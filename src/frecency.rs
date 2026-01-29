use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_AGE: f64 = 10000.0;
const HOUR: u64 = 3600;
const DAY: u64 = 86400;
const WEEK: u64 = 604800;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub score: f64,
    pub last: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Store {
    pub entries: HashMap<String, Entry>,
}

pub fn path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("f")
        .join("frecency.json")
}

pub fn load() -> Result<Store> {
    let p = path();
    if !p.exists() {
        return Ok(Store::default());
    }
    let content = fs::read_to_string(&p)?;
    let store: Store = serde_json::from_str(&content)?;
    Ok(store)
}

pub fn save(store: &Store) -> Result<()> {
    let p = path();
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string(store)?;
    fs::write(&p, content)?;
    Ok(())
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn boost(store: &mut Store, path: &str) {
    let ts = now();
    let entry = store.entries.entry(path.to_string()).or_insert(Entry {
        score: 0.0,
        last: ts,
    });
    entry.score += 1.0;
    entry.last = ts;
    age(store);
}

pub fn age(store: &mut Store) {
    let total: f64 = store.entries.values().map(|e| e.score).sum();
    if total <= MAX_AGE {
        return;
    }
    let factor = (MAX_AGE * 0.9) / total;
    store.entries.retain(|_, e| {
        e.score *= factor;
        e.score >= 1.0
    });
}

pub fn prune(store: &mut Store) {
    let ts = now();
    let ninety_days = 90 * DAY;
    store.entries.retain(|p, e| {
        let exists = std::path::Path::new(p).exists();
        let old = ts.saturating_sub(e.last) > ninety_days;
        exists || !old
    });
}

pub fn frecency(entry: &Entry) -> f64 {
    let ts = now();
    let age = ts.saturating_sub(entry.last);
    let multiplier = if age < HOUR {
        4.0
    } else if age < DAY {
        2.0
    } else if age < WEEK {
        0.5
    } else {
        0.25
    };
    entry.score * multiplier
}

pub fn score(store: &Store, path: &str) -> f64 {
    store
        .entries
        .get(path)
        .map(|e| frecency(e))
        .unwrap_or(0.0)
}
