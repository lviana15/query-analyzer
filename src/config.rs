use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use serde::Deserialize;

const CONFIG_FILE_NAME: &str = "redshift.config.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Warning
    }
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzerConfig {
    #[serde(default)]
    pub defaults: ConfigDefaults,
    #[serde(default)]
    pub collections: Vec<CollectionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigDefaults {
    pub unknown_collection_severity: Option<Severity>,
    pub recommended_predicate_miss_severity: Option<Severity>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionConfig {
    pub name: String,
    #[serde(default)]
    pub high_volume: bool,
    #[serde(default)]
    pub indexes: Vec<IndexConfig>,
    pub predicate_guidance: Option<PredicateGuidance>,
    pub performance: Option<PerformanceConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexConfig {
    pub name: Option<String>,
    pub keys: Vec<IndexKey>,
    pub unique: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexKey {
    pub field: String,
    pub order: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredicateGuidance {
    #[serde(default)]
    pub recommended_fields: Vec<String>,
    pub severity: Option<Severity>,
    #[serde(default)]
    pub methods: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceConfig {
    pub unbounded_find: Option<PerformanceRule>,
    pub sort_without_index: Option<PerformanceRule>,
    pub regex_without_prefix: Option<PerformanceRule>,
    pub max_fields_in_predicate: Option<ThresholdRule>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceRule {
    pub severity: Option<Severity>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdRule {
    pub value: usize,
    pub severity: Option<Severity>,
}

#[derive(Debug, Clone)]
pub struct LoadedConfig {
    pub path: PathBuf,
    pub config: AnalyzerConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigWarning {
    pub severity: Severity,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
}

impl ConfigDefaults {
    pub fn unknown_collection_severity(&self) -> Severity {
        self.unknown_collection_severity.unwrap_or_default()
    }

    pub fn recommended_predicate_miss_severity(&self) -> Severity {
        self.recommended_predicate_miss_severity.unwrap_or_default()
    }
}

pub fn load_project_config(
    project_dir: &Path,
) -> Result<Option<LoadedConfig>, Box<dyn std::error::Error>> {
    let Some(config_path) = discover_config(project_dir) else {
        return Ok(None);
    };

    let content = fs::read_to_string(&config_path)?;
    let config: AnalyzerConfig = serde_json::from_str(&content).map_err(|err| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Failed to parse {}: {}", config_path.display(), err),
        )
    })?;

    validate_config(&config).map_err(|err| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Invalid {}: {}", config_path.display(), err),
        )
    })?;

    Ok(Some(LoadedConfig {
        path: config_path,
        config,
    }))
}

fn discover_config(start_dir: &Path) -> Option<PathBuf> {
    let mut current = Some(start_dir);
    while let Some(dir) = current {
        let candidate = dir.join(CONFIG_FILE_NAME);
        if candidate.is_file() {
            return Some(candidate);
        }
        current = dir.parent();
    }
    None
}

fn validate_config(config: &AnalyzerConfig) -> Result<(), String> {
    let mut collections = HashSet::new();
    for collection in &config.collections {
        if collection.name.trim().is_empty() {
            return Err("collection name must not be empty".to_string());
        }

        if !collections.insert(collection.name.clone()) {
            return Err(format!("duplicate collection '{}'", collection.name));
        }

        for index in &collection.indexes {
            if index.keys.is_empty() {
                return Err(format!(
                    "collection '{}' has index with no keys",
                    collection.name
                ));
            }

            for key in &index.keys {
                if key.field.trim().is_empty() {
                    return Err(format!(
                        "collection '{}' has index key with empty field name",
                        collection.name
                    ));
                }
                if key.order != 1 && key.order != -1 {
                    return Err(format!(
                        "collection '{}' field '{}' has invalid order {} (allowed: 1 or -1)",
                        collection.name, key.field, key.order
                    ));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::{discover_config, AnalyzerConfig};

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{}_{}", prefix, nanos))
    }

    #[test]
    fn parses_basic_config() {
        let content = r#"
        {
          "defaults": {
            "unknownCollectionSeverity": "warning"
          },
          "collections": [
            {
              "name": "users",
              "indexes": [
                {
                  "keys": [
                    { "field": "email", "order": 1 }
                  ]
                }
              ],
              "predicateGuidance": {
                "recommendedFields": ["organizationId"]
              }
            }
          ]
        }
        "#;

        let parsed: AnalyzerConfig = serde_json::from_str(content).unwrap();
        assert_eq!(parsed.collections.len(), 1);
        assert_eq!(parsed.collections[0].name, "users");
    }

    #[test]
    fn discovers_config_in_parent_directory() {
        let root = unique_temp_dir("redshift_config_test");
        let nested = root.join("services").join("api");
        fs::create_dir_all(&nested).unwrap();

        let config_path = root.join("redshift.config.json");
        fs::write(&config_path, "{}").unwrap();

        let discovered = discover_config(&nested);
        assert_eq!(discovered, Some(config_path));

        fs::remove_dir_all(&root).unwrap();
    }
}
