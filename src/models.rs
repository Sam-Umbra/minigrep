use std::{
    fmt::{self, Debug, Display, Formatter},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Parameters {
    #[arg(short, long)]
    pub query: String,

    #[arg(short, long = "file-path")]
    pub file_path: Option<String>,

    #[arg(short, long = "ignore-case")]
    pub ignore_case: bool,

    #[arg(short, long = "whole-word")]
    pub whole_word: bool,

    #[arg(short, long, num_args = 0..=1, default_missing_value = ".")]
    pub directory: Option<PathBuf>,

    #[arg(short, long)]
    pub simple_search: bool,
    // TODO: Directory search
    // Also return the file name

    // TODO: Simple search
    // Returns how many times the word was found, in directory search also return the file name

    // TODO: Normal Search
    // Should return the line number where the word was found, in directory search returns file name and lines

    // TODO: Integrate Threading (Use Rayon crate)
}

pub struct FileMatchModel {
    pub file_name: String,
    pub lines: Vec<LineMatchModel>,
}

impl FileMatchModel {
    pub fn new(file_name: String, lines: Vec<LineMatchModel>) -> Self {
        FileMatchModel { file_name, lines }
    }
}

impl Display for FileMatchModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File: {}", self.file_name)?;
        for line in &self.lines {
            write!(f, "\n{}", line)?;
        }
        Ok(())
    }
}

pub struct LineMatchModel {
    pub line: usize,
    pub content: String,
}

impl LineMatchModel {
    pub fn new(line: usize, content: String) -> Self {
        LineMatchModel { line, content }
    }
}

impl Display for LineMatchModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Line {}: {}", self.line, self.content)
    }
}

/* impl Parameters {
    pub fn build<T>(mut args: T) -> Result<Parameters, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Parameters {
            query,
            file_path,
            ignore_case,
        })
    }
} */
