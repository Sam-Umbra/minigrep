use minigrep::{search, search_case_insensitive};
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let params = Parameters::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(params) {
        println!("Application error: {e}");
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

struct Parameters {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Parameters {
    fn build(args: &[String]) -> Result<Parameters, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Parameters {
            query,
            file_path,
            ignore_case,
        })
    }
}
