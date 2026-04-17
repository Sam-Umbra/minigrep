pub struct Parameters {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Parameters {
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
}

pub enum flags {
    
}