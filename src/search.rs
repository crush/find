pub fn find(directories: &[String], query: &str) -> Vec<String> {
    let query_lower = query.to_lowercase();

    let mut exact: Vec<&String> = Vec::new();
    let mut prefix: Vec<&String> = Vec::new();
    let mut contains: Vec<&String> = Vec::new();

    for dir in directories {
        let name = dir.rsplit('/').next().unwrap_or(dir);
        let name_lower = name.to_lowercase();

        if name_lower == query_lower {
            exact.push(dir);
        } else if name_lower.starts_with(&query_lower) {
            prefix.push(dir);
        } else if name_lower.contains(&query_lower) {
            contains.push(dir);
        }
    }

    exact
        .into_iter()
        .chain(prefix)
        .chain(contains)
        .cloned()
        .collect()
}
