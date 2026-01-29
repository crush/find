use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher};

pub fn find(directories: &[String], query: &str) -> Vec<String> {
    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);
    let query_lower = query.to_lowercase();

    let mut scored: Vec<(u32, &String)> = directories
        .iter()
        .filter_map(|dir| {
            let name = dir.rsplit('/').next().unwrap_or(dir);
            let name_lower = name.to_lowercase();

            if name_lower == query_lower {
                return Some((u32::MAX, dir));
            }

            if name_lower.starts_with(&query_lower) {
                return Some((u32::MAX - 1, dir));
            }

            let mut buf = Vec::new();
            let haystack = nucleo_matcher::Utf32Str::new(name, &mut buf);
            pattern.score(haystack, &mut matcher).map(|score| (score, dir))
        })
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));

    scored.into_iter().map(|(_, dir)| dir.clone()).collect()
}
