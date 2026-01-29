mod config;
mod frecency;
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
    Boost { path: String },
    Prune,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Index) => index::run(),
        Some(Commands::List) => list_roots(),
        Some(Commands::Add { path }) => config::add_root(&path),
        Some(Commands::Remove { path }) => config::remove_root(&path),
        Some(Commands::Boost { path }) => boost(&path),
        Some(Commands::Prune) => prune(),
        None => {
            if cli.query.is_empty() {
                index::run()
            } else {
                jump(&cli.query.join(" "))
            }
        }
    };

    if let Err(e) = result {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn jump(query: &str) -> Result<()> {
    let cache = index::load_cache()?;
    let mut store = frecency::load()?;

    frecency::prune(&mut store);

    let dirs: Vec<String> = cache
        .directories
        .into_iter()
        .filter(|d| std::path::Path::new(d).exists())
        .collect();

    if dirs.is_empty() {
        eprintln!("no directories indexed");
        return Ok(());
    }

    let matches = search::find(&dirs, query, &store);

    if matches.is_empty() {
        return Ok(());
    }

    let path = if matches.len() == 1 {
        Some(matches[0].clone())
    } else {
        ui::select(&matches)
    };

    if let Some(p) = path {
        println!("{p}");
    }
    Ok(())
}

fn boost(path: &str) -> Result<()> {
    let expanded = shellexpand::tilde(path).to_string();
    let mut store = frecency::load()?;
    frecency::boost(&mut store, &expanded);
    frecency::save(&store)?;
    Ok(())
}

fn prune() -> Result<()> {
    let mut store = frecency::load()?;
    let before = store.entries.len();
    frecency::prune(&mut store);
    let after = store.entries.len();
    frecency::save(&store)?;
    eprintln!("pruned {} entries", before - after);
    Ok(())
}

fn list_roots() -> Result<()> {
    let config = config::load()?;
    for root in &config.roots {
        eprintln!("{root}");
    }
    Ok(())
}
