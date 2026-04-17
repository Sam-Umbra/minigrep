use minigrep::{parameter::Parameters, search, search_case_insensitive};
use std::{env, error::Error, fs, process};

fn main() {
    let params = Parameters::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(params) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(params: Parameters) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(params.file_path)?;

    let results = if params.ignore_case {
        search_case_insensitive(&params.query, &file_contents)
    } else {
        search(&params.query, &file_contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
