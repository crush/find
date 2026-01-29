use crate::config;
use anyhow::Result;
use jwalk::WalkDir;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const IGNORED: &[&str] = &[
    "node_modules",
    "target",
    "dist",
    "build",
    "vendor",
    "__pycache__",
    ".git",
    ".next",
    ".turbo",
    ".cache",
    ".npm",
    ".pnpm",
    "coverage",
    ".nyc_output",
];

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cache {
    pub directories: Vec<String>,
}

pub fn cache_path() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("f")
        .join("cache.json")
}

pub fn load_cache() -> Result<Cache> {
    let path = cache_path();

    if !path.exists() {
        return Ok(Cache::default());
    }

    let content = fs::read_to_string(&path)?;
    let cache: Cache = serde_json::from_str(&content)?;
    Ok(cache)
}

pub fn save_cache(cache: &Cache) -> Result<()> {
    let path = cache_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(cache)?;
    fs::write(&path, content)?;
    Ok(())
}

pub fn run() -> Result<()> {
    let config = config::load()?;

    if config.roots.is_empty() {
        let home = dirs::home_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());

        println!("no roots configured. add one with:");
        println!("  f add {home}");
        println!("  f add {home}/code");
        return Ok(());
    }

    println!("indexing...");

    let directories: Vec<String> = config
        .roots
        .par_iter()
        .flat_map(|root| scan(root))
        .collect();

    let cache = Cache { directories };
    let count = cache.directories.len();
    save_cache(&cache)?;

    println!("indexed {count} directories");
    Ok(())
}

fn scan(root: &str) -> Vec<String> {
    let mut dirs = Vec::new();

    for entry in WalkDir::new(root)
        .max_depth(5)
        .skip_hidden(true)
        .process_read_dir(|_, _, _, children| {
            children.retain(|e| {
                e.as_ref()
                    .map(|entry| {
                        let name = entry.file_name().to_str().unwrap_or("");
                        !IGNORED.contains(&name)
                    })
                    .unwrap_or(false)
            });
        })
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();
        if is_project(&path) {
            dirs.push(path.to_string_lossy().to_string());
        }
    }

    dirs
}

fn is_project(path: &std::path::Path) -> bool {
    let markers = [
        ".git",
        "package.json",
        "Cargo.toml",
        "go.mod",
        "pyproject.toml",
        "setup.py",
        "Makefile",
        "CMakeLists.txt",
        "pom.xml",
        "build.gradle",
        "mix.exs",
        "deno.json",
        "bun.lockb",
    ];

    markers.iter().any(|m| path.join(m).exists())
}
