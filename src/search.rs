use crate::frecency::Store;

pub struct Match {
    pub path: String,
    pub rank: f64,
}

pub fn find(directories: &[String], query: &str, store: &Store) -> Vec<String> {
    let terms: Vec<String> = query.to_lowercase().split_whitespace().map(String::from).collect();
    if terms.is_empty() {
        return vec![];
    }

    let mut matches: Vec<Match> = directories
        .iter()
        .filter_map(|dir| score(dir, &terms, store))
        .collect();

    matches.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap_or(std::cmp::Ordering::Equal));
    matches.into_iter().map(|m| m.path).collect()
}

fn score(path: &str, terms: &[String], store: &Store) -> Option<Match> {
    let path_lower = path.to_lowercase();
    let name = path.rsplit('/').next().unwrap_or(path);
    let name_lower = name.to_lowercase();

    if !matches_terms(&path_lower, terms) {
        return None;
    }

    let mut rank = crate::frecency::score(store, path);

    let last_term = terms.last()?;
    if name_lower == *last_term {
        rank += 1000.0;
    } else if name_lower.starts_with(last_term) {
        rank += 500.0;
    } else if name_lower.contains(last_term) {
        rank += 100.0;
    }

    if terms.len() == 1 && name_lower == *last_term {
        rank += 2000.0;
    }

    Some(Match {
        path: path.to_string(),
        rank,
    })
}

fn matches_terms(path: &str, terms: &[String]) -> bool {
    let mut pos = 0;
    for term in terms {
        match path[pos..].find(term) {
            Some(i) => pos += i + term.len(),
            None => return false,
        }
    }
    true
}
