use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 2 {
        //     return Err("Not enough arguments");
        // }
        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(file) => file,
            None => return Err("Didn't receive a file path"),
        };
        let ignore_case = match args.next() {
            Some(value) => {
                if value.contains(&String::from("-i")) {
                    true
                } else {
                    false
                }
            }
            None => match std::env::var("IGNORE_CASE") {
                Ok(ignore_case) => !(ignore_case.is_empty() || ignore_case == "0"),
                Err(_) => false,
            },
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = std::fs::read_to_string(&self.file_path)?;

        if self.ignore_case {
            search_case_insensitive(&self.query, &contents).map(|result| println!("{result}"));
        } else {
            search(&self.query, &contents).map(|result| println!("{result}"));
        };

        Ok(())
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query.to_lowercase()))
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
        let mut iter = search(query, contents);
        assert_eq!("safe, fast, productive.", iter.next().unwrap());
    }

    #[test]
    fn case_insenstive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let mut iter = search_case_insensitive(query, contents);
        assert_eq!("Rust:", iter.next().unwrap());
        assert_eq!("Trust me.", iter.next().unwrap());
    }
}
