use crate::MongoQuery;
use std::collections::HashMap;
use swc_core::common::{sync::Lrc, FileName, SourceMap, SourceMapper, Spanned};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};

const QUERY_METHODS: &[&str] = &[
    "find",
    "findOne",
    "findOneAndUpdate",
    "findOneAndDelete",
    "findOneAndReplace",
    "updateOne",
    "updateMany",
    "deleteOne",
    "deleteMany",
    "aggregate",
    "count",
    "countDocuments",
    "distinct",
];

pub struct MongoQueryVisitor<'a> {
    pub queries: Vec<MongoQuery>,
    pub source_map: &'a SourceMap,
    pub file_path: String,
    pub model_map: HashMap<String, String>,
    pub local_variables: HashMap<String, ObjectLit>,
}

impl<'a> MongoQueryVisitor<'a> {
    pub fn new(source_map: &'a SourceMap, file_path: String) -> Self {
        Self {
            queries: Vec::new(),
            source_map,
            file_path,
            model_map: HashMap::new(),
            local_variables: HashMap::new(),
        }
    }

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
                        if let Some(key) = get_prop_key(&kv.key) {
                            if key.starts_with('$') {
                                match &*kv.value {
                                    Expr::Object(nested_obj) => {
                                        self.extract_fields_recursive(nested_obj, fields)
                                    }
                                    Expr::Array(arr) => {
                                        for elem in arr.elems.iter().flatten() {
                                            if let Expr::Object(nested_obj) = &*elem.expr {
                                                self.extract_fields_recursive(nested_obj, fields);
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            } else {
                                fields.push(key);
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

    fn analyze_callee(&self, callee: &Callee) -> Option<(String, String)> {
        let Callee::Expr(expr) = callee else {
            return None;
        };
        let Expr::Member(member_expr) = &**expr else {
            return None;
        };

        let method_name = get_member_prop_name(&member_expr.prop)?;
        if !QUERY_METHODS.contains(&method_name.as_str()) {
            return None;
        }

        let collection = self.resolve_collection(&member_expr.obj)?;
        Some((collection, method_name))
    }

    fn resolve_collection(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Call(call_expr) => {
                let Callee::Expr(callee_expr) = &call_expr.callee else {
                    return None;
                };
                let Expr::Member(member) = &**callee_expr else {
                    return None;
                };

                let prop = get_member_prop_name(&member.prop)?;
                if prop == "collection" {
                    if let Some(Expr::Lit(Lit::Str(s))) =
                        call_expr.args.first().map(|arg| &*arg.expr)
                    {
                        return Some(s.value.as_str().unwrap_or_default().to_string());
                    }
                } else if prop == "useDb" || prop == "getConnection" {
                    return self.resolve_collection(&member.obj);
                }
                None
            }
            Expr::Member(member) => {
                if let Expr::This(_) = &*member.obj {
                    let prop = get_member_prop_name(&member.prop)?;
                    if let Some(collection) = self.model_map.get(&prop) {
                        return Some(collection.clone());
                    }
                    if prop.ends_with("Model") {
                        return Some(prop.replace("Model", "").to_lowercase());
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
        for prop in n.params.iter().filter_map(|p| match p {
            ParamOrTsParamProp::TsParamProp(prop) => Some(prop),
            _ => None,
        }) {
            let TsParamPropParam::Ident(ident) = &prop.param else {
                continue;
            };
            let prop_name = ident.sym.as_str();

            if let Some(model_name) = prop.decorators.iter().find_map(get_injected_model_name) {
                self.model_map.insert(prop_name.to_string(), model_name);
            }
        }
        n.visit_children_with(self);
    }

    fn visit_var_decl(&mut self, n: &VarDecl) {
        for decl in &n.decls {
            if let (Some(init), Pat::Ident(binding)) = (&decl.init, &decl.name) {
                if let Expr::Object(obj) = &**init {
                    self.local_variables
                        .insert(binding.id.sym.as_str().to_string(), obj.clone());
                }
            }
        }
        n.visit_children_with(self);
    }

    fn visit_call_expr(&mut self, n: &CallExpr) {
        n.visit_children_with(self);

        if let Some((collection, method)) = self.analyze_callee(&n.callee) {
            let args_to_check: &[usize] = match method.as_str() {
                "find" | "findOne" | "count" | "countDocuments" | "deleteMany" | "deleteOne"
                | "aggregate" => &[0],
                "distinct" => &[1],
                "updateOne" | "updateMany" | "findOneAndUpdate" | "findOneAndReplace" => &[0],
                _ => &[],
            };

            let mut fields = Vec::new();

            for &arg_idx in args_to_check {
                if let Some(arg) = n.args.get(arg_idx) {
                    match &*arg.expr {
                        Expr::Object(obj) => fields.extend(self.extract_fields(obj)),
                        Expr::Ident(ident) => {
                            if let Some(obj) = self.local_variables.get(ident.sym.as_str()) {
                                fields.extend(self.extract_fields(obj));
                            }
                        }
                        Expr::Array(arr) => {
                            for elem in arr.elems.iter().flatten() {
                                if let Expr::Object(obj) = &*elem.expr {
                                    fields.extend(self.extract_fields(obj));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            fields.sort();
            fields.dedup();

            let loc = self.source_map.lookup_char_pos(n.span.lo);

            let predicate_arg_idx = if method.as_str() == "distinct" { 1 } else { 0 };

            let predicate = if let Some(arg) = n.args.get(predicate_arg_idx) {
                self.source_map
                    .span_to_snippet(arg.span())
                    .unwrap_or_else(|_| "...".to_string())
            } else {
                "".to_string()
            };

            let raw_match = format!("{}.{}({})", collection, method, predicate);

            self.queries.push(MongoQuery {
                file: self.file_path.clone(),
                line: loc.line,
                collection,
                method,
                query_fields: fields,
                raw_match,
            });
        }
    }
}

fn get_prop_key(key: &PropName) -> Option<String> {
    match key {
        PropName::Ident(ident) => Some(ident.sym.as_str().to_string()),
        PropName::Str(s) => Some(s.value.as_str().unwrap_or_default().to_string()),
        PropName::Computed(c) => {
            if let Expr::Lit(Lit::Str(s)) = &*c.expr {
                Some(s.value.as_str().unwrap_or_default().to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_member_prop_name(prop: &MemberProp) -> Option<String> {
    match prop {
        MemberProp::Ident(ident) => Some(ident.sym.as_str().to_string()),
        _ => None,
    }
}

fn get_injected_model_name(decorator: &Decorator) -> Option<String> {
    let Expr::Call(call) = &*decorator.expr else {
        return None;
    };
    let Callee::Expr(callee_expr) = &call.callee else {
        return None;
    };
    let Expr::Ident(callee) = &**callee_expr else {
        return None;
    };

    if callee.sym != "InjectModel" {
        return None;
    }

    let arg = call.args.first()?;
    match &*arg.expr {
        Expr::Member(mem) => {
            let Expr::Ident(obj) = &*mem.obj else {
                return None;
            };
            Some(obj.sym.as_str().to_string())
        }
        Expr::Lit(Lit::Str(s)) => Some(s.value.as_str().unwrap_or_default().to_string()),
        _ => None,
    }
}

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

    let module = parser.parse_module().map_or_else(
        |_| Module {
            span: Default::default(),
            body: Vec::new(),
            shebang: None,
        },
        |m| m,
    );

    let mut visitor = MongoQueryVisitor::new(&cm, file_path.to_string());
    module.visit_with(&mut visitor);

    visitor.queries
}
