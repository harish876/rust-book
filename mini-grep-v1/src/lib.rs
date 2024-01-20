use std::{env::var, error::Error, fs};

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }

        let filename = &args[1];
        let query = &args[2];

        Ok(Self {
            query: query.to_string(),
            filename: filename.to_string(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let res = search(&config.query, &contents);
    for val in res {
        println!("{}", val);
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let is_case_insensitive = var("CASE_INSENSITIVE").is_err();

    let contents = contents.lines();
    let mut res: Vec<&str> = vec![];
    for content in contents {
        if is_case_insensitive {
            if content.to_lowercase().contains(&query.to_lowercase()) {
                res.push(content);
            }
        }

        if content.contains(&query) {
            res.push(content);
        }
    }

    res
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust is
very
safe and productive";

        assert_eq!(vec!["safe and productive"], search(query, contents));
    }
}
