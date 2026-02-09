use clap::{Parser, Subcommand};
use mongo_analyzer::{analyze_project, get_collection_analysis, get_indexes};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mongo-analyzer")]
#[command(about = "Tool to analyze MongoDB queries in TypeScript projects")]
struct Cli {
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        #[arg(short, long)]
        verbose: bool,
    },
    Indexes,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { verbose } => {
            let results = analyze_project(&cli.directory)?;
            let analysis = get_collection_analysis(&results);

            for collection_data in &analysis {
                println!("Collection: {}", collection_data.collection);

                for file_data in &collection_data.files {
                    let file_name = std::path::Path::new(&file_data.file_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&file_data.file_path);

                    println!("  File: {}", file_name);

                    for pattern in &file_data.patterns {
                        let fields_str = if pattern.fields.is_empty() {
                            "[no fields]".to_string()
                        } else {
                            format!("[{}]", pattern.fields.join(", "))
                        };
                        println!("    - {}: {} usages", fields_str, pattern.count);

                        if verbose {
                            let mut queries = pattern.queries.clone();
                            queries.sort_by_key(|q| q.line);
                            for query in queries {
                                println!("      Line {}: {}", query.line, query.raw_match);
                            }
                        }
                    }
                }
                println!();
            }
        }
        Commands::Indexes => {
            let results = analyze_project(&cli.directory)?;
            let indexes = get_indexes(&results);

            for (collection, suggestions) in indexes {
                println!("Collection: {}", collection);
                for suggestion in &suggestions {
                    println!("   {}", suggestion);
                }
                println!();
            }
        }
    }

    Ok(())
}
