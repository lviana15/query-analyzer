use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub services: HashMap<String, ServiceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub patterns: Vec<String>,
    pub excluded_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MongoQuery {
    pub file: String,
    pub line: usize,
    pub service: String,
    pub collection: String,
    pub method: String,
    pub query_fields: Vec<String>,
    pub raw_match: String,
}

#[derive(Debug, Clone)]
pub struct FieldUsageInfo {
    pub field: String,
    pub total_usage: usize,
    pub usage_by_file: HashMap<String, usize>,
}

impl FieldUsageInfo {
    pub fn new(field: String) -> Self {
        Self {
            field,
            total_usage: 0,
            usage_by_file: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CollectionFieldAnalysis {
    pub collection: String,
    pub total_queries: usize,
    pub files_accessing: Vec<(String, usize)>,
    pub field_usage: Vec<FieldUsageInfo>,
}

// Internal struct for data aggregation
#[derive(Debug, Clone)]
struct CollectionData {
    file_usage: HashMap<String, usize>,
    field_usage: HashMap<String, FieldUsageInfo>,
}

impl CollectionData {
    pub fn new() -> Self {
        Self {
            file_usage: HashMap::new(),
            field_usage: HashMap::new(),
        }
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        let mut services = HashMap::new();
        services.insert(
            "user-service".to_string(),
            ServiceConfig {
                name: "user-service".to_string(),
                patterns: vec!["user.*\\.ts".to_string(), "auth.*\\.ts".to_string()],
                excluded_files: vec!["*.spec.ts".to_string(), "*.test.ts".to_string()],
            },
        );
        services.insert(
            "product-service".to_string(),
            ServiceConfig {
                name: "product-service".to_string(),
                patterns: vec!["product.*\\.ts".to_string(), "catalog.*\\.ts".to_string()],
                excluded_files: vec!["*.spec.ts".to_string(), "*.test.ts".to_string()],
            },
        );

        ProjectConfig { services }
    }
}

pub fn analyze_project(
    root_dir: &Path,
    config: Option<ProjectConfig>,
) -> Result<Vec<MongoQuery>, Box<dyn std::error::Error>> {
    let config = config.unwrap_or_default();
    let mut queries = Vec::new();

    // Find all TypeScript files
    let ts_files = find_ts_files(root_dir)?;

    for file_path in ts_files {
        let content = fs::read_to_string(&file_path)?;
        let file_path_str = file_path.to_string_lossy().to_string();

        // Determine which service this file belongs to
        let service_name = determine_service(&file_path_str, &config);

        // Skip if service not in config
        if !config.services.contains_key(&service_name) {
            continue;
        }

        // Check if file should be excluded
        let service_config = &config.services[&service_name];
        if should_exclude_file(&file_path_str, service_config) {
            continue;
        }

        // Find MongoDB queries using multi-line detection
        let file_queries = find_mongo_queries(&content, &file_path_str, &service_name);
        queries.extend(file_queries);
    }

    // Deduplicate queries based on file, line, and method
    queries.sort_by(|a, b| {
        a.file
            .cmp(&b.file)
            .then(a.line.cmp(&b.line))
            .then(a.method.cmp(&b.method))
    });
    queries.dedup();

    Ok(queries)
}

fn find_mongo_queries(content: &str, file_path: &str, service_name: &str) -> Vec<MongoQuery> {
    let mut queries = Vec::new();

    // Updated MongoDB patterns for TypeScript - ordered to avoid overlaps
    let mongo_patterns = [
        // Pattern 1: Model access: this.productModel.find() - most specific first
        Regex::new(r"this\.([a-z]\w*Model)\.(find|findOne|aggregate|count|countDocuments|distinct)\(").unwrap(),
        // Pattern 2: connection.useDb().collection().find()
        Regex::new(r#"connection\.useDb\([^)]+\)\.collection\(\s*['"]([^'"]+)['"]\s*\)\.(find|findOne|aggregate|count|countDocuments|distinct)\("#).unwrap(),
        // Pattern 3: Direct collection access: collection("name").find()
        Regex::new(r#"(?:\w+\.)?collection\(\s*['"]([^'"]+)['"]\s*\)\.(find|findOne|aggregate|count|countDocuments|distinct)\("#).unwrap(),
        // Pattern 4: this.collection.find()
        Regex::new(r"this\.collection\.(find|findOne|aggregate|count|countDocuments|distinct)\(").unwrap(),
        // Pattern 5: Repository pattern
        Regex::new(r"repository\.(find|findOne|aggregate|count|countDocuments|distinct)\(").unwrap(),
    ];

    let lines: Vec<&str> = content.lines().collect();
    let mut matched_lines = std::collections::HashSet::new();

    for (line_idx, line) in lines.iter().enumerate() {
        // Skip if this line was already matched by a more specific pattern
        if matched_lines.contains(&line_idx) {
            continue;
        }

        for (pattern_idx, pattern) in mongo_patterns.iter().enumerate() {
            if let Some(caps) = pattern.captures(line) {
                let method = caps
                    .get(if pattern_idx == 0 || pattern_idx == 1 {
                        2
                    } else {
                        1
                    })
                    .map(|m| m.as_str())
                    .unwrap_or("unknown");

                let collection = extract_collection_name_from_pattern(line, pattern_idx)
                    .unwrap_or_else(|| "unknown".to_string());

                // Extract multi-line query object
                let query_start_line = line_idx;
                let (query_fields, query_end_line) =
                    extract_fields_multiline(&lines, query_start_line);

                // Create the raw match string
                let raw_match = lines[query_start_line..=query_end_line.min(lines.len() - 1)]
                    .iter()
                    .map(|&s| s.trim())
                    .collect::<Vec<_>>()
                    .join(" ");

                queries.push(MongoQuery {
                    file: file_path.to_string(),
                    line: query_start_line + 1,
                    service: service_name.to_string(),
                    collection,
                    method: method.to_string(),
                    query_fields,
                    raw_match,
                });

                // Mark this line as matched to prevent duplicates
                matched_lines.insert(line_idx);
                break; // Stop checking other patterns for this line
            }
        }
    }

    queries
}

fn extract_fields_multiline(lines: &[&str], start_line: usize) -> (Vec<String>, usize) {
    let mut brace_count = 0;
    let mut in_object = false;
    let mut end_line = start_line;

    // Find the opening brace on the start line or subsequent lines
    for (line_offset, &line) in lines[start_line..].iter().enumerate() {
        end_line = start_line + line_offset;

        // Count braces to track object boundaries
        for ch in line.chars() {
            if ch == '{' {
                brace_count += 1;
                in_object = true;
            } else if ch == '}' {
                brace_count -= 1;
                if brace_count == 0 && in_object {
                    // End of object found, extract fields from this range
                    let object_content = lines[start_line..=end_line]
                        .iter()
                        .map(|&s| s.trim())
                        .collect::<Vec<_>>()
                        .join(" ");

                    let fields = extract_fields_from_object(&object_content);
                    return (fields, end_line);
                }
            }
        }

        // If we've gone too far without finding a complete object, break
        if line_offset > 10 && brace_count == 0 {
            break;
        }
    }

    // If no complete object found, try to extract from what we have
    let partial_content = lines[start_line..=end_line.min(lines.len() - 1)]
        .iter()
        .map(|&s| s.trim())
        .collect::<Vec<_>>()
        .join(" ");

    let fields = extract_fields_from_object(&partial_content);
    (fields, end_line)
}

fn extract_fields_from_object(content: &str) -> Vec<String> {
    let mut fields = Vec::new();

    // Pattern to find field names in object literals
    let field_pattern = Regex::new(r#"([a-zA-Z_]\w*)\s*:"#).unwrap();

    // Find all field name matches
    for caps in field_pattern.captures_iter(content) {
        if let Some(field_match) = caps.get(1) {
            let field = field_match.as_str();
            // Filter out common non-field identifiers
            if field != "new"
                && field != "Object"
                && field != "this"
                && !field.starts_with("$")
                && !fields.contains(&field.to_string())
            {
                fields.push(field.to_string());
            }
        }
    }

    // Handle special MongoDB operators and extract field names from them
    let operator_patterns = [
        Regex::new(r#"\$or\s*:\s*\[\s*\{([^}]+)\}"#).unwrap(),
        Regex::new(r#"\$and\s*:\s*\[\s*\{([^}]+)\}"#).unwrap(),
        Regex::new(r#"\$regex\s*:\s*['"]([^'"]+)['"]"#).unwrap(),
    ];

    for pattern in &operator_patterns {
        for caps in pattern.captures_iter(content) {
            if let Some(field_match) = caps.get(1) {
                let field_content = field_match.as_str();
                // Extract field names from operator content
                for field_caps in field_pattern.captures_iter(field_content) {
                    if let Some(f) = field_caps.get(1) {
                        let field = f.as_str();
                        if !fields.contains(&field.to_string()) {
                            fields.push(field.to_string());
                        }
                    }
                }
            }
        }
    }

    fields.sort();
    fields
}

fn extract_collection_name_from_pattern(line: &str, pattern_idx: usize) -> Option<String> {
    match pattern_idx {
        0 => {
            // this.productModel pattern - extract from model name
            let re = Regex::new(r"this\.([a-z]\w*Model)").unwrap();
            re.captures(line).map(|caps| {
                let model_name = &caps[1];
                // Remove "Model" suffix and convert to lowercase
                model_name.replace("Model", "").to_lowercase()
            })
        }
        1 => {
            // connection.useDb().collection().find() pattern
            let re = Regex::new(r#"collection\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
            re.captures(line).map(|caps| caps[1].to_string())
        }
        2 => {
            // collection("name") pattern
            let re = Regex::new(r#"collection\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
            re.captures(line).map(|caps| caps[1].to_string())
        }
        3 => {
            // this.collection pattern
            Some("this.collection".to_string())
        }
        4 => {
            // repository pattern
            let re = Regex::new(r"repository\.(\w+)").unwrap();
            re.captures(line)
                .map(|caps| format!("repository.{}", caps[1].to_lowercase()))
        }
        _ => None,
    }
}

fn find_ts_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Skip node_modules and .git directories
            if let Some(name) = path.file_name() {
                if name == "node_modules" || name == ".git" || name == "dist" {
                    continue;
                }
            }
            files.extend(find_ts_files(&path)?);
        } else if let Some(extension) = path.extension() {
            if extension == "ts" || extension == "tsx" {
                files.push(path);
            }
        }
    }

    Ok(files)
}

fn determine_service(file_path: &str, config: &ProjectConfig) -> String {
    for (service_name, service_config) in &config.services {
        for pattern in &service_config.patterns {
            let regex = Regex::new(pattern)
                .unwrap_or_else(|_| Regex::new(&regex::escape(pattern)).unwrap());

            if regex.is_match(file_path) {
                return service_name.clone();
            }
        }
    }

    // Default service based on directory structure
    if file_path.contains("user") || file_path.contains("auth") {
        "user-service".to_string()
    } else if file_path.contains("product") || file_path.contains("catalog") {
        "product-service".to_string()
    } else {
        "unknown-service".to_string()
    }
}

fn should_exclude_file(file_path: &str, config: &ServiceConfig) -> bool {
    for pattern in &config.excluded_files {
        if file_path.contains(&pattern.replace("*", "")) {
            return true;
        }
    }
    false
}

pub fn get_detailed_field_analysis(
    queries: &[MongoQuery],
    service_filter: Option<&str>,
) -> Vec<CollectionFieldAnalysis> {
    let mut collection_data: HashMap<String, CollectionData> = HashMap::new();

    // First pass: aggregate data
    for query in queries {
        if let Some(service) = service_filter {
            if query.service != service {
                continue;
            }
        }

        let coll_data = collection_data
            .entry(query.collection.clone())
            .or_insert_with(|| CollectionData::new());

        // Track file usage
        *coll_data.file_usage.entry(query.file.clone()).or_insert(0) += 1;

        // Track field usage
        for field in &query.query_fields {
            let field_info = coll_data
                .field_usage
                .entry(field.clone())
                .or_insert_with(|| FieldUsageInfo::new(field.clone()));
            field_info.total_usage += 1;
            *field_info
                .usage_by_file
                .entry(query.file.clone())
                .or_insert(0) += 1;
        }
    }

    // Second pass: create final analyses with sorting
    let mut analyses: Vec<CollectionFieldAnalysis> = collection_data
        .into_iter()
        .map(|(collection, data)| {
            let mut files: Vec<_> = data
                .file_usage
                .into_iter()
                .map(|(file, count)| (file, count))
                .collect();
            files.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by query count descending

            let mut fields: Vec<_> = data.field_usage.into_iter().map(|(_, info)| info).collect();
            fields.sort_by(|a, b| b.total_usage.cmp(&a.total_usage)); // Sort by usage descending

            CollectionFieldAnalysis {
                collection,
                total_queries: files.iter().map(|(_, count)| count).sum(),
                files_accessing: files,
                field_usage: fields,
            }
        })
        .collect();

    // Sort collections alphabetically
    analyses.sort_by(|a, b| a.collection.cmp(&b.collection));
    analyses
}

pub fn get_fields(
    queries: &[MongoQuery],
    service_filter: Option<&str>,
) -> HashMap<String, Vec<String>> {
    let mut fields_by_collection = HashMap::new();

    for query in queries {
        if let Some(service) = service_filter {
            if query.service != service {
                continue;
            }
        }

        let collection_fields = fields_by_collection
            .entry(query.collection.clone())
            .or_insert_with(Vec::new);

        for field in &query.query_fields {
            if !collection_fields.contains(field) {
                collection_fields.push(field.clone());
            }
        }
    }

    fields_by_collection
}

pub fn get_methods(
    queries: &[MongoQuery],
    service_filter: Option<&str>,
) -> HashMap<String, Vec<String>> {
    let mut methods_by_collection = HashMap::new();

    for query in queries {
        if let Some(service) = service_filter {
            if query.service != service {
                continue;
            }
        }

        let collection_methods = methods_by_collection
            .entry(query.collection.clone())
            .or_insert_with(Vec::new);

        if !collection_methods.contains(&query.method) {
            collection_methods.push(query.method.clone());
        }
    }

    methods_by_collection
}

pub fn get_indexes(
    queries: &[MongoQuery],
    service_filter: Option<&str>,
) -> HashMap<String, Vec<String>> {
    let mut suggestions_by_collection = HashMap::new();

    let mut usage_counts: HashMap<String, HashMap<String, usize>> = HashMap::new();

    // Count field usage
    for query in queries {
        if let Some(service) = service_filter {
            if query.service != service {
                continue;
            }
        }

        let collection_usage = usage_counts.entry(query.collection.clone()).or_default();

        for field in &query.query_fields {
            *collection_usage.entry(field.clone()).or_insert(0) += 1;
        }
    }

    // Generate suggestions
    for (collection, field_usage) in usage_counts {
        let mut suggestions = Vec::new();

        // Find frequently used fields
        let mut sorted_fields: Vec<_> = field_usage.iter().collect();
        sorted_fields.sort_by(|a, b| b.1.cmp(a.1));

        if let Some((field, count)) = sorted_fields.first() {
            if **count > 2 {
                suggestions.push(format!(
                    "Consider indexing field '{}' (used {} times)",
                    field, count
                ));
            }
        }

        // Suggest compound indexes for multiple fields
        if sorted_fields.len() > 2 {
            let top_fields: Vec<_> = sorted_fields
                .iter()
                .take(3)
                .map(|(field, _)| field.as_str())
                .collect();
            suggestions.push(format!(
                "Consider compound index on fields: {}",
                top_fields.join(", ")
            ));
        }

        if !suggestions.is_empty() {
            suggestions_by_collection.insert(collection, suggestions);
        }
    }

    suggestions_by_collection
}
