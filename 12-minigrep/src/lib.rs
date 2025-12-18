use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        let query = args[0].clone();
        let file_path = args[1].clone();
        let ignore_case;
        if args.len() == 3 {
            if args.contains(&String::from("-i")) {
                ignore_case = true;
            } else {
                ignore_case = false;
            }
        } else {
            ignore_case = match std::env::var("IGNORE_CASE") {
                Ok(ignore_case) => !ignore_case.is_empty() && ignore_case != "0",
                Err(_) => false,
            };
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = std::fs::read_to_string(&self.file_path)?;

        let results = if self.ignore_case {
            search_case_insensitive(&self.query, &contents)
        } else {
            search(&self.query, &contents)
        };

        for line in results {
            println!("{line}");
        }
        Ok(())
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    // unimplemented!("Implement search function");
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insenstive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
