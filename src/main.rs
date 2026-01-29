mod config;
mod index;
mod search;
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "f", about = "instant directory jumper")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(trailing_var_arg = true)]
    query: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    Index,
    List,
    Add { path: String },
    Remove { path: String },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Index) => index::run(),
        Some(Commands::List) => list_roots(),
        Some(Commands::Add { path }) => config::add_root(&path),
        Some(Commands::Remove { path }) => config::remove_root(&path),
        None => {
            if cli.query.is_empty() {
                index::run()
            } else {
                jump(&cli.query.join(" "))
            }
        }
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn jump(query: &str) -> Result<()> {
    let cache = index::load_cache()?;

    if cache.directories.is_empty() {
        eprintln!("no directories indexed. run 'f index' first");
        return Ok(());
    }

    let matches = search::find(&cache.directories, query);

    if matches.is_empty() {
        eprintln!("no matches for '{query}'");
        return Ok(());
    }

    let path = if matches.len() == 1 {
        matches[0].clone()
    } else {
        ui::select(&matches)?
    };

    println!("{path}");
    Ok(())
}

fn list_roots() -> Result<()> {
    let config = config::load()?;
    for root in &config.roots {
        eprintln!("{root}");
    }
    Ok(())
}
