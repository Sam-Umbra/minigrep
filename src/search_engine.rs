use std::{borrow::Cow, fs, path::PathBuf};

use walkdir::WalkDir;

use crate::models::{FileMatchModel, LineMatchModel, Parameters};

pub fn search_file(params: &Parameters, file_contents: &str) -> Vec<LineMatchModel> {
    let query: Cow<str> = if params.ignore_case {
        Cow::Owned(params.query.to_lowercase())
    } else {
        Cow::Borrowed(&params.query)
    };

    file_contents
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let normalized_line: Cow<str> = if params.ignore_case {
                Cow::Owned(line.to_lowercase())
            } else {
                Cow::Borrowed(line)
            };

            let matched = if params.whole_word {
                is_whole_word_match(&normalized_line, &query)
            } else {
                normalized_line.contains(query.as_ref())
            };

            if matched {
                Some(LineMatchModel::new(i + 1, line.to_string()))
            } else {
                None
            }
        })
        .collect()
}

pub fn search_directory(dir: &PathBuf, params: &Parameters) -> Vec<FileMatchModel> {
    let mut result: Vec<FileMatchModel> = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
            match fs::read_to_string(path) {
                Ok(content) => {
                    let lines = search_file(params, &content);
                    if !lines.is_empty() {
                        let file_name = path.to_string_lossy().to_string();
                        result.push(FileMatchModel::new(file_name, lines));
                    }
                },
                Err(e) => eprintln!("Couldn't read {:?}: {}", path, e),
            }
        }
    }

    result
}

fn is_whole_word_match(line: &str, query: &str) -> bool {
    if query.is_empty() {
        return false;
    }

    line.match_indices(query).any(|(start, _)| {
        let end = start + query.len();

        let before_ok = line[..start]
            .chars()
            .next_back()
            .map_or(true, |c| !c.is_alphanumeric() && c != '_');

        let after_ok = line[end..]
            .chars()
            .next()
            .map_or(true, |c| !c.is_alphanumeric() && c != '_');

        before_ok && after_ok
    })
}

/* pub fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub fn search_whole_word<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|&line| line == query)
        .collect()
} */
