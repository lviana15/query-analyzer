use clap::{Parser, Subcommand};
use mongo_analyzer::{analyze_project, get_indexes};
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
    Analyze,
    Indexes,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze => {
            let results = analyze_project(&cli.directory)?;

            println!("   Total queries found: {}\n", results.len());

            for result in &results {
                let file_name = std::path::Path::new(&result.file)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&result.file);

                println!(
                    "File: {}:{} ({})",
                    file_name, result.line, result.collection
                );
                println!("  Method: {}", result.method);
                println!("  Fields: {}\n", result.query_fields.join(", "));
            }

            println!("Summary:");
            println!("   Total queries: {}", results.len());
            let unique_collections: std::collections::HashSet<_> =
                results.iter().map(|r| &r.collection).collect();
            println!("   Unique collections: {}", unique_collections.len());
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
