pub mod parameter;

pub fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
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
