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

#[derive(Debug, Clone)]
pub struct MongoQuery {
    pub file: String,
    pub line: usize,
    pub service: String,
    pub collection: String,
    pub method: String,
    pub query_fields: Vec<String>,
    pub raw_match: String,
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

    // Compile MongoDB patterns
    let mongo_patterns = [
        // Pattern 1: Direct collection access: collection.find()
        Regex::new(
            r"(?:\w+\.)?collection\([^)]+\)\.(find|findOne|aggregate|count|countDocuments|distinct)\(",
        )?,
        // Pattern 2: Model access: User.find()
        Regex::new(r"([A-Z]\w*)\.(find|findOne|aggregate|count|countDocuments|distinct)\(")?,
        // Pattern 3: connection.useDb().collection().find()
        Regex::new(
            r"connection\.useDb\([^)]+\)\.collection\([^)]+\)\.(find|findOne|aggregate|count|countDocuments|distinct)\(",
        )?,
        // Pattern 4: This pattern: this.collection.find()
        Regex::new(r"this\.collection\.(find|findOne|aggregate|count|countDocuments|distinct)\(")?,
        // Pattern 5: Repository pattern: repository.find()
        Regex::new(r"repository\.(find|findOne|aggregate|count|countDocuments|distinct)\(")?,
    ];

    // Field extraction patterns
    let field_patterns = [
        // Extract object literal fields: { field: value, field2: value2 }
        Regex::new(r"\{\s*([^}]+)\s*\}")?,
        // Extract variable references: field1, field2
        Regex::new(r"[a-zA-Z_]\w*")?,
    ];

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

        // Find MongoDB queries
        for (line_num, line) in content.lines().enumerate() {
            for (pattern_idx, pattern) in mongo_patterns.iter().enumerate() {
                if let Some(caps) = pattern.captures(line) {
                    let method = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");

                    let collection = extract_collection_name(line, pattern_idx)
                        .unwrap_or_else(|| "unknown".to_string());

                    let query_fields = extract_fields_from_line(line, &field_patterns);

                    queries.push(MongoQuery {
                        file: file_path_str.clone(),
                        line: line_num + 1,
                        service: service_name.clone(),
                        collection,
                        method: method.to_string(),
                        query_fields,
                        raw_match: line.to_string(),
                    });
                }
            }
        }
    }

    Ok(queries)
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

fn extract_collection_name(line: &str, pattern_idx: usize) -> Option<String> {
    match pattern_idx {
        0 => {
            // collection("name") pattern
            let re = Regex::new(r#"collection\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
            re.captures(line).map(|caps| caps[1].to_string())
        }
        1 => {
            // Model.find() pattern - extract model name
            let re = Regex::new(r"([A-Z]\w*)\.(find|findOne|aggregate)").unwrap();
            re.captures(line)
                .map(|caps| caps[1].to_string().to_lowercase())
        }
        2 => {
            // connection.useDb().collection().find() pattern
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

fn extract_fields_from_line(line: &str, field_patterns: &[Regex]) -> Vec<String> {
    let mut fields = Vec::new();

    // Look for object literals in the line
    if let Some(obj_caps) = field_patterns[0].captures(line) {
        let obj_content = &obj_caps[1];

        // Split by comma and extract field names
        for part in obj_content.split(',') {
            let part = part.trim();
            if let Some(field_match) = field_patterns[1].captures(part) {
                let field = field_match[0].trim();
                if !field.is_empty() && field != "new" && field != "Object" {
                    fields.push(field.to_string());
                }
            }
        }
    }

    // Remove duplicates and sort
    fields.sort();
    fields.dedup();
    fields
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
