mod config;
mod frecency;
mod index;
mod search;
mod ui;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::io;
use std::process;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
    Top,
    Mark { name: String, path: Option<String> },
    Unmark { name: String },
    Marks,
    Back { query: Option<String> },
    Completions { shell: Shell },
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
        Some(Commands::Top) => top(),
        Some(Commands::Mark { name, path }) => mark(&name, path.as_deref()),
        Some(Commands::Unmark { name }) => unmark(&name),
        Some(Commands::Marks) => list_marks(),
        Some(Commands::Back { query }) => back(query.as_deref()),
        Some(Commands::Completions { shell }) => completions(shell),
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
    let cfg = config::load()?;

    if let Some(path) = cfg.marks.get(query) {
        if std::path::Path::new(path).exists() {
            println!("{path}");
            return Ok(());
        }
    }

    let cache = index::load_cache()?;
    let store = frecency::load()?;

    let dirs: Vec<String> = cache
        .directories
        .into_iter()
        .filter(|d| std::path::Path::new(d).exists())
        .collect();

    if dirs.is_empty() {
        return Ok(());
    }

    let matches = search::find(&dirs, query, &store);

    if matches.is_empty() {
        return Ok(());
    }

    let limited: Vec<(String, u32)> = matches.into_iter().take(20).collect();

    let path = if limited.len() == 1 {
        Some(limited[0].0.clone())
    } else {
        ui::select(&limited)
    };

    if let Some(p) = path {
        println!("{p}");
    }
    Ok(())
}

fn back(query: Option<&str>) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let mut current = cwd.as_path();

    while let Some(parent) = current.parent() {
        if let Some(q) = query {
            let name = parent.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.to_lowercase().contains(&q.to_lowercase()) {
                println!("{}", parent.display());
                return Ok(());
            }
        } else if parent.join(".git").exists() {
            println!("{}", parent.display());
            return Ok(());
        }
        current = parent;
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
    eprintln!("{}", before - after);
    Ok(())
}

fn top() -> Result<()> {
    let store = frecency::load()?;
    let mut entries: Vec<_> = store
        .entries
        .iter()
        .map(|(p, e)| (p, frecency::frecency(e)))
        .collect();
    entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    for (path, score) in entries.iter().take(10) {
        let name = path.rsplit('/').next().unwrap_or(path);
        eprintln!("{:>6.0}  {}", score, name);
    }
    Ok(())
}

fn mark(name: &str, path: Option<&str>) -> Result<()> {
    let mut cfg = config::load()?;
    let target = match path {
        Some(p) => shellexpand::tilde(p).to_string(),
        None => std::env::current_dir()?.to_string_lossy().to_string(),
    };
    cfg.marks.insert(name.to_string(), target.clone());
    config::save(&cfg)?;
    eprintln!("{name} -> {target}");
    Ok(())
}

fn unmark(name: &str) -> Result<()> {
    let mut cfg = config::load()?;
    cfg.marks.remove(name);
    config::save(&cfg)?;
    Ok(())
}

fn list_marks() -> Result<()> {
    let cfg = config::load()?;
    for (name, path) in &cfg.marks {
        eprintln!("{name} -> {path}");
    }
    Ok(())
}

fn list_roots() -> Result<()> {
    let cfg = config::load()?;
    for root in &cfg.roots {
        eprintln!("{root}");
    }
    Ok(())
}

fn completions(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "f", &mut io::stdout());
    Ok(())
}
