use clap::Parser;
use std::{error::Error, fs, process};
use ugrep::{
    models::Parameters,
    search_engine::{search_directory, search_file},
};

fn main() {
    /* let params = Parameters::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    }); */
    let params = Parameters::parse();

    if let Err(e) = run(params) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(params: Parameters) -> Result<(), Box<dyn Error>> {
    if let Some(dir) = &params.directory {
        for file_match in search_directory(dir, &params) {
            println!("{file_match}\n");
        }
    } else {
        let path = params
            .file_path
            .as_deref()
            .ok_or("File path is required when not using --directory")?;
        let file_contents = fs::read_to_string(path)?;
        for line_match in search_file(&params, &file_contents) {
            println!("{line_match}");
        }
    }

    Ok(())
}
