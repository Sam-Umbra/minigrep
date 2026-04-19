use clap::Parser;
use riops::{
    models::Parameters,
    search_engine::{search_directory, search_file},
};
use std::{error::Error, process};

fn main() {
    // Parse CLI arguments via clap's derive macro.
    let params = Parameters::parse();

    if let Err(e) = run(params) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/// Executes the search based on the provided [`Parameters`] and prints results to stdout.
///
/// Dispatches to either single-file or directory mode depending on whether
/// `params.directory` is set. Two output modes are supported, controlled by
/// `params.simple_search`:
///
/// | Mode   | Output                                                             |
/// |--------|--------------------------------------------------------------------|
/// | Normal | Every matching line prefixed with its 1-based line number.         |
/// | Simple | One summary line per file: `<query> in <file>: N occurrences(s)`  |
///
/// # Examples
///
/// ```bash
/// # Normal mode — shows each matching line
/// rps -q "error" -f ./app.log
/// # Line 3: error: connection refused
/// # Line 7: error: timeout
///
/// # Simple mode — shows a count per file
/// rps -q "error" -d ./logs --simple-search
/// # error in ./logs/app.log: 2 occurrences(s)
/// # error in ./logs/system.log: 5 occurrences(s)
/// ```
///
/// # Errors
///
/// Returns an error if:
/// * `--directory` is not set and no `--file-path` is provided.
/// * The specified file cannot be opened or read.
fn run(params: Parameters) -> Result<(), Box<dyn Error>> {
    if let Some(dir) = &params.directory {
        // --- Directory mode (parallel) ---
        let file_matches = search_directory(dir, &params);

        if params.simple_search {
            // One summary line per matched file.
            for file_match in &file_matches {
                println!(
                    "{} in {}: {} occurrences(s)",
                    params.query,
                    file_match.file_name,
                    file_match.lines.len()
                )
            }
        } else {
            // Full output: file header followed by every matching line.
            for file_match in file_matches {
                println!("{file_match}\n");
            }
        }
    } else {
        // --- Single-file mode ---
        let path = params
            .file_path
            .as_deref()
            .ok_or("File path is required when not using --directory")?;

        let line_matches = search_file(&params, path)?;

        if params.simple_search {
            // One-line occurrence count.
            println!("{}: {} Occurrences(s)", params.query, line_matches.len())
        } else {
            // Print each matching line.
            for line_match in line_matches {
                println!("{line_match}");
            }
        }
    }

    Ok(())
}
