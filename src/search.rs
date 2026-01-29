use crate::frecency::Store;
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher};

pub fn find(directories: &[String], query: &str, store: &Store) -> Vec<(String, u32)> {
    if query.is_empty() {
        return vec![];
    }

    let mut matcher = Matcher::new(Config::DEFAULT);
    let pattern = Pattern::parse(query, CaseMatching::Smart, Normalization::Smart);

    let mut matches: Vec<(String, u32)> = directories
        .iter()
        .filter_map(|dir| {
            let mut buf = Vec::new();
            let haystack = nucleo_matcher::Utf32Str::new(dir, &mut buf);
            pattern.score(haystack, &mut matcher).map(|score| {
                let frecency = crate::frecency::score(store, dir) as u32;
                let combined = score.saturating_add(frecency);
                (dir.clone(), combined)
            })
        })
        .collect();

    matches.sort_by(|a, b| b.1.cmp(&a.1));
    matches
}
