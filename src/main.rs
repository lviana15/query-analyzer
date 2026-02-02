use clap::{Parser, Subcommand};
use mongo_analyzer::{analyze_project, get_fields, get_indexes, get_methods, ProjectConfig};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mongo-analyzer")]
#[command(about = "A CLI tool to analyze MongoDB queries in TypeScript projects")]
struct Cli {
    /// Directory to analyze (default: current directory)
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze the entire project for MongoDB usage patterns
    Analyze {
        /// Service name to focus on
        #[arg(short, long)]
        service: Option<String>,
    },
    /// Get all fields used in queries
    Fields {
        /// Service name to filter by
        #[arg(short, long)]
        service: Option<String>,
    },
    /// Get all MongoDB methods used
    Methods {
        /// Service name to filter by
        #[arg(short, long)]
        service: Option<String>,
    },
    /// Generate index suggestions
    Indexes {
        /// Service name to filter by
        #[arg(short, long)]
        service: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load config
    let config = if let Some(config_path) = &cli.config {
        let content = std::fs::read_to_string(config_path)?;
        Some(serde_json::from_str::<ProjectConfig>(&content)?)
    } else {
        None
    };

    match cli.command {
        Commands::Analyze { service } => {
            let results = analyze_project(&cli.directory, config)?;

            if let Some(service_name) = service {
                let service_results: Vec<_> = results
                    .iter()
                    .filter(|r| r.service == service_name)
                    .collect();

                println!("🔍 Analysis for service: {}", service_name);
                println!("Found {} MongoDB queries\n", service_results.len());

                for result in &service_results {
                    println!("📄 {}:{} ({})", result.file, result.line, result.collection);
                    println!("   Method: {}", result.method);
                    println!("   Fields: {}\n", result.query_fields.join(", "));
                }
            } else {
                // Group by service
                let mut services = std::collections::HashMap::new();
                for result in &results {
                    services
                        .entry(&result.service)
                        .or_insert_with(Vec::new)
                        .push(result);
                }

                println!("🔍 MongoDB Query Analysis Results");
                println!("=====================================\n");

                for (service, queries) in services {
                    println!("📦 Service: {}", service);
                    println!("   Queries found: {}\n", queries.len());

                    for query in queries {
                        println!("   📄 {}:{} ({})", query.file, query.line, query.collection);
                        println!("      Method: {}", query.method);
                        println!("      Fields: {}", query.query_fields.join(", "));
                    }
                    println!();
                }
            }

            // Summary
            println!("📊 Summary:");
            println!("   Total queries: {}", results.len());
            let unique_collections: std::collections::HashSet<_> =
                results.iter().map(|r| &r.collection).collect();
            println!("   Unique collections: {}", unique_collections.len());
            let unique_services: std::collections::HashSet<_> =
                results.iter().map(|r| &r.service).collect();
            println!("   Services using MongoDB: {}", unique_services.len());
        }
        Commands::Fields { service } => {
            let results = analyze_project(&cli.directory, config)?;
            let fields = get_fields(&results, service.as_deref());

            println!("🔍 Field Analysis");
            println!("==================\n");

            for (collection, field_list) in fields {
                println!("📄 Collection: {}", collection);
                for field in &field_list {
                    println!("   • {}", field);
                }
                println!();
            }
        }
        Commands::Methods { service } => {
            let results = analyze_project(&cli.directory, config)?;
            let methods = get_methods(&results, service.as_deref());

            println!("🔍 Method Usage");
            println!("===============\n");

            for (collection, method_list) in methods {
                println!("📄 Collection: {}", collection);
                for method in &method_list {
                    println!("   • {}", method);
                }
                println!();
            }
        }
        Commands::Indexes { service } => {
            let results = analyze_project(&cli.directory, config)?;
            let indexes = get_indexes(&results, service.as_deref());

            println!("🔍 Index Suggestions");
            println!("====================\n");

            for (collection, suggestions) in indexes {
                println!("📄 Collection: {}", collection);
                for suggestion in &suggestions {
                    println!("   💡 {}", suggestion);
                }
                println!();
            }
        }
    }

    Ok(())
}
