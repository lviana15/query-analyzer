use crate::MongoQuery;
use std::collections::HashMap;
use swc_core::common::{sync::Lrc, FileName, SourceMap};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};

pub struct MongoQueryVisitor<'a> {
    pub queries: Vec<MongoQuery>,
    pub source_map: &'a SourceMap,
    pub file_path: String,
    pub model_map: HashMap<String, String>,
}

impl<'a> MongoQueryVisitor<'a> {
    pub fn new(source_map: &'a SourceMap, file_path: String) -> Self {
        Self {
            queries: Vec::new(),
            source_map,
            file_path,
            model_map: HashMap::new(),
        }
    }

    // Helper to extract fields from an object literal
    fn extract_fields(&self, obj: &ObjectLit) -> Vec<String> {
        let mut fields = Vec::new();
        self.extract_fields_recursive(obj, &mut fields);
        fields.sort();
        fields.dedup();
        fields
    }

    fn extract_fields_recursive(&self, obj: &ObjectLit, fields: &mut Vec<String>) {
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                match &**prop {
                    Prop::KeyValue(kv) => {
                        let key = self.get_prop_key(&kv.key);

                        if let Some(k) = key {
                            // If key is an operator (starts with $), recurse into value
                            if k.starts_with('$') {
                                if let Expr::Object(nested_obj) = &*kv.value {
                                    self.extract_fields_recursive(nested_obj, fields);
                                } else if let Expr::Array(arr) = &*kv.value {
                                    for elem in &arr.elems {
                                        if let Some(expr) = elem {
                                            if let Expr::Object(nested_obj) = &*expr.expr {
                                                self.extract_fields_recursive(nested_obj, fields);
                                            }
                                        }
                                    }
                                }
                            } else {
                                // It's a real field
                                fields.push(k);
                            }
                        }
                    }
                    Prop::Shorthand(ident) => {
                        fields.push(ident.sym.as_str().to_string());
                    }
                    _ => {}
                }
            }
        }
    }

    fn get_prop_key(&self, key: &PropName) -> Option<String> {
        match key {
            PropName::Ident(ident) => Some(ident.sym.as_str().to_string()),
            PropName::Str(s) => Some(s.value.as_str().unwrap_or_default().to_string()),
            PropName::Computed(c) => {
                // Handle computed properties if they are string literals
                if let Expr::Lit(Lit::Str(s)) = &*c.expr {
                    Some(s.value.as_str().unwrap_or_default().to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // Helper to analyze the callee chain and determine collection/method
    fn analyze_callee(&self, callee: &Callee) -> Option<(String, String)> {
        if let Callee::Expr(expr) = callee {
            self.analyze_callee_expr(expr)
        } else {
            None
        }
    }

    fn analyze_callee_expr(&self, callee: &Expr) -> Option<(String, String)> {
        // We expect something like: object.method(...)
        // method is the MongoDB method (find, insertOne, etc.)
        // object determines the collection

        if let Expr::Member(member_expr) = callee {
            if let Some(method_name) = self.get_member_prop_name(&member_expr.prop) {
                if !self.is_query_method(&method_name) {
                    return None;
                }

                // Analyze the object to find collection
                if let Some(collection) = self.resolve_collection(&member_expr.obj) {
                    return Some((collection, method_name));
                }
            }
        }
        None
    }

    fn is_query_method(&self, method: &str) -> bool {
        matches!(
            method,
            "find"
                | "findOne"
                | "findOneAndUpdate"
                | "findOneAndDelete"
                | "findOneAndReplace"
                | "insertOne"
                | "insertMany"
                | "updateOne"
                | "updateMany"
                | "deleteOne"
                | "deleteMany"
                | "aggregate"
                | "count"
                | "countDocuments"
                | "distinct"
        )
    }

    fn get_member_prop_name(&self, prop: &MemberProp) -> Option<String> {
        match prop {
            MemberProp::Ident(ident) => Some(ident.sym.as_str().to_string()),
            _ => None,
        }
    }

    fn resolve_collection(&self, expr: &Expr) -> Option<String> {
        match expr {
            // Case 1: .collection('name')
            Expr::Call(call_expr) => {
                if let Callee::Expr(callee_expr) = &call_expr.callee {
                    if let Expr::Member(member) = &**callee_expr {
                        if let Some(prop) = self.get_member_prop_name(&member.prop) {
                            if prop == "collection" {
                                // Extract argument
                                if let Some(arg) = call_expr.args.first() {
                                    if let Expr::Lit(Lit::Str(s)) = &*arg.expr {
                                        return Some(
                                            s.value.as_str().unwrap_or_default().to_string(),
                                        );
                                    }
                                }
                            } else if prop == "useDb" || prop == "getConnection" {
                                // Chain continuation, keep going down
                                return self.resolve_collection(&member.obj);
                            }
                        }
                    }
                }
                None
            }
            // Case 2: this.productModel
            Expr::Member(member) => {
                // Check if accessing `this`
                if let Expr::This(_) = &*member.obj {
                    if let Some(prop) = self.get_member_prop_name(&member.prop) {
                        // Lookup in model map
                        if let Some(collection) = self.model_map.get(&prop) {
                            return Some(collection.clone());
                        }

                        // Fallback: heuristic if map lookup failed (e.g. inferred from name)
                        if prop.ends_with("Model") {
                            return Some(prop.replace("Model", "").to_lowercase());
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}

impl<'a> Visit for MongoQueryVisitor<'a> {
    fn visit_constructor(&mut self, n: &Constructor) {
        // Inspect constructor parameters for @InjectModel
        for param in &n.params {
            if let ParamOrTsParamProp::TsParamProp(prop) = param {
                // Get the property name
                let prop_name = match &prop.param {
                    TsParamPropParam::Ident(ident) => Some(ident.sym.as_str().to_string()),
                    _ => None,
                };

                if let Some(name) = prop_name {
                    // Check decorators
                    for decorator in &prop.decorators {
                        if let Expr::Call(call) = &*decorator.expr {
                            if let Callee::Expr(callee_expr) = &call.callee {
                                if let Expr::Ident(callee) = &**callee_expr {
                                    if callee.sym.as_str() == "InjectModel" {
                                        // Extract model name from argument
                                        if let Some(arg) = call.args.first() {
                                            // Case: @InjectModel(Product.name) -> MemberExpr
                                            if let Expr::Member(mem) = &*arg.expr {
                                                if let Expr::Ident(obj) = &*mem.obj {
                                                    self.model_map.insert(
                                                        name.clone(),
                                                        obj.sym.as_str().to_string(),
                                                    );
                                                }
                                            }
                                            // Case: @InjectModel('Product') -> String Literal
                                            else if let Expr::Lit(Lit::Str(s)) = &*arg.expr {
                                                self.model_map.insert(
                                                    name.clone(),
                                                    s.value
                                                        .as_str()
                                                        .unwrap_or_default()
                                                        .to_string(),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        n.visit_children_with(self);
    }

    fn visit_call_expr(&mut self, n: &CallExpr) {
        // Visit children first to handle nested calls (arguments)
        n.visit_children_with(self);

        if let Some((collection, method)) = self.analyze_callee(&n.callee) {
            let mut fields = Vec::new();

            // Extract fields from arguments
            // Logic varies by method
            let args_to_check = match method.as_str() {
                "find" | "findOne" | "count" | "countDocuments" | "deleteMany" | "deleteOne"
                | "distinct" => vec![0], // Query is 1st arg
                "insertOne" | "insertMany" => vec![0], // Doc is 1st arg
                "updateOne" | "updateMany" | "findOneAndUpdate" | "findOneAndReplace" => vec![0, 1], // Query is 1st, Update is 2nd
                "aggregate" => vec![0], // Pipeline is 1st (array of objects)
                _ => vec![],
            };

            for arg_idx in args_to_check {
                if let Some(arg) = n.args.get(arg_idx) {
                    match &*arg.expr {
                        Expr::Object(obj) => {
                            let extracted = self.extract_fields(obj);
                            fields.extend(extracted);
                        }
                        Expr::Array(arr) => {
                            // For aggregate pipeline
                            for elem in &arr.elems {
                                if let Some(e) = elem {
                                    if let Expr::Object(obj) = &*e.expr {
                                        let extracted = self.extract_fields(obj);
                                        fields.extend(extracted);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            fields.sort();
            fields.dedup();

            // Get line number
            let loc = self.source_map.lookup_char_pos(n.span.lo);
            let line = loc.line;

            // Generate "raw match" (simplified reconstruction or just placeholder)
            let raw_match = format!("{}.{}(...)", collection, method);

            self.queries.push(MongoQuery {
                file: self.file_path.clone(),
                line: line,
                collection,
                method,
                query_fields: fields,
                raw_match,
            });
        }
    }
}

// Minimal no-op emitter to suppress errors during parsing
pub fn parse_file(content: &str, file_path: &str) -> Vec<MongoQuery> {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        FileName::Custom(file_path.to_string()).into(),
        content.to_string(),
    );

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            decorators: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    // We don't really care about parse errors for now, just try to get what we can
    // but parser.parse_module() stops on first error often.
    // However, SWC is robust.

    let module = match parser.parse_module() {
        Ok(m) => m,
        Err(_) => {
            // If main parse fails, we might miss everything.
            // But usually for valid TS files it's fine.
            return Vec::new();
        }
    };

    let mut visitor = MongoQueryVisitor::new(&cm, file_path.to_string());
    module.visit_with(&mut visitor);

    visitor.queries
}
