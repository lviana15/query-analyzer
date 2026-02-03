use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub mod ast_parser;

#[derive(Debug, Clone, PartialEq)]
pub struct MongoQuery {
    pub file: String,
    pub line: usize,
    pub collection: String,
    pub method: String,
    pub query_fields: Vec<String>,
    pub raw_match: String,
}

#[derive(Debug, PartialEq)]
pub enum IndexSuggestion {
    SingleField { field: String, count: usize },
    Compound { fields: Vec<String> },
}

impl fmt::Display for IndexSuggestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingleField { field, count } => {
                write!(
                    f,
                    "Consider indexing field '{}' (used {} times)",
                    field, count
                )
            }
            Self::Compound { fields } => {
                write!(
                    f,
                    "Consider compound index on fields: {}",
                    fields.join(", ")
                )
            }
        }
    }
}

pub fn analyze_project(root_dir: &Path) -> Result<Vec<MongoQuery>, Box<dyn std::error::Error>> {
    let ts_files = find_ts_files(root_dir);

    let queries: Vec<MongoQuery> = ts_files
        .into_iter()
        .filter_map(|path| match fs::read_to_string(&path) {
            Ok(content) => {
                let path_str = path.to_string_lossy();
                Some(find_mongo_queries(&content, &path_str))
            }
            Err(e) => {
                eprintln!("Warning: Failed to read file {}: {}", path.display(), e);
                None
            }
        })
        .flatten()
        .collect();

    let mut queries = queries;
    queries.sort_by(|a, b| {
        a.file
            .cmp(&b.file)
            .then(a.line.cmp(&b.line))
            .then(a.method.cmp(&b.method))
    });
    queries.dedup();

    Ok(queries)
}

fn find_mongo_queries(content: &str, file_path: &str) -> Vec<MongoQuery> {
    ast_parser::parse_file(content, file_path)
}

fn find_ts_files(dir: &Path) -> Vec<PathBuf> {
    const IGNORED_DIRS: &[&str] = &["node_modules", ".git", "dist", "target"];

    WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| {
            let file_name = e.file_name().to_str().unwrap_or("");
            !IGNORED_DIRS.contains(&file_name)
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map_or(false, |ext| ext == "ts" || ext == "tsx")
        })
        .filter(|e| {
            let path_str = e.path().to_string_lossy();
            !path_str.ends_with(".spec.ts") && !path_str.ends_with(".test.ts")
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

pub fn get_indexes(queries: &[MongoQuery]) -> HashMap<String, Vec<IndexSuggestion>> {
    let mut suggestions_by_collection = HashMap::new();
    let usage_counts = count_field_usage(queries);

    for (collection, field_usage) in usage_counts {
        let suggestions = generate_suggestions_for_collection(&field_usage);
        if !suggestions.is_empty() {
            suggestions_by_collection.insert(collection, suggestions);
        }
    }

    suggestions_by_collection
}

fn count_field_usage(queries: &[MongoQuery]) -> HashMap<String, HashMap<String, usize>> {
    let mut usage_counts: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for query in queries {
        let collection_usage = usage_counts.entry(query.collection.clone()).or_default();
        for field in &query.query_fields {
            *collection_usage.entry(field.clone()).or_insert(0) += 1;
        }
    }

    usage_counts
}

fn generate_suggestions_for_collection(
    field_usage: &HashMap<String, usize>,
) -> Vec<IndexSuggestion> {
    let mut suggestions = Vec::new();

    let mut sorted_fields: Vec<_> = field_usage.iter().collect();
    sorted_fields.sort_by(|a, b| b.1.cmp(a.1));

    if let Some((field, count)) = sorted_fields.first() {
        if **count > 2 {
            suggestions.push(IndexSuggestion::SingleField {
                field: field.to_string(),
                count: **count,
            });
        }
    }

    if sorted_fields.len() > 2 {
        let top_fields: Vec<_> = sorted_fields
            .iter()
            .take(3)
            .map(|(field, _)| field.to_string())
            .collect();
        suggestions.push(IndexSuggestion::Compound { fields: top_fields });
    }

    suggestions
}
