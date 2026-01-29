use crate::config;
use anyhow::Result;
use ignore::WalkBuilder;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cache {
    pub directories: Vec<String>,
}

pub fn cache_path() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("f")
        .join("cache")
}

pub fn load_cache() -> Result<Cache> {
    let path = cache_path();

    if !path.exists() {
        return Ok(Cache::default());
    }

    let data = fs::read(&path)?;
    let cache: Cache = bincode::deserialize(&data).unwrap_or_default();
    Ok(cache)
}

pub fn save_cache(cache: &Cache) -> Result<()> {
    let path = cache_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let data = bincode::serialize(cache)?;
    fs::write(&path, data)?;
    Ok(())
}

pub fn run() -> Result<()> {
    let config = config::load()?;

    if config.roots.is_empty() {
        let home = dirs::home_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());

        eprintln!("f add {home}");
        return Ok(());
    }

    use std::io::{stderr, Write};

    let counter = AtomicUsize::new(0);

    let directories: Vec<String> = config
        .roots
        .par_iter()
        .flat_map(|root| {
            let dirs = scan(root);
            let prev = counter.fetch_add(dirs.len(), Ordering::Relaxed);
            eprint!("\r\x1b[K{}", prev + dirs.len());
            let _ = stderr().flush();
            dirs
        })
        .collect();

    let cache = Cache { directories };
    save_cache(&cache)?;

    eprintln!();
    Ok(())
}

fn scan(root: &str) -> Vec<String> {
    let mut dirs = Vec::new();

    let walker = WalkBuilder::new(root)
        .max_depth(Some(5))
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry in walker.filter_map(|e| e.ok()) {
        if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }

        let path = entry.path();
        if is_project(path) {
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
        "flake.nix",
        "shell.nix",
        "Project.toml",
        "pubspec.yaml",
        "Package.swift",
    ];

    markers.iter().any(|m| path.join(m).exists())
}
