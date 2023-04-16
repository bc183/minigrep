use std::{error::Error, fs, path::Path};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    println!("Results");
    if config.case_sensitive {
        let res = search(&config.query, &contents);
        for (line_num, line) in res {
            println!("{:?}. {}", line_num, line);
        }
    }
    if !config.case_sensitive {
        let res = isearch(&config.query, &contents);
        for (line_num, line) in res {
            println!("{:?}. {}", line_num, line);
        }
    }
    Ok(())
}

pub struct Config<'a> {
    pub path: &'a Path,
    pub query: String,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() != 4 {
            return Err("Arguments must be path query case_sensitive");
        }
        let path = Path::new(&args[1]);
        let case_sensitive = args[3].clone();
        let case_sensitive = case_sensitive.as_str();
        Ok(Config {
            path,
            query: args[2].clone(),
            case_sensitive: match case_sensitive {
                "true" => true,
                "false" => false,
                _ => false,
            },
        })
    }

    pub fn path_exists(&self) -> bool {
        return self.path.exists();
    }
}

pub fn search<'a>(search: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut result: Vec<(usize, &'a str)> = Vec::new();
    for (idx, line) in contents.lines().into_iter().enumerate() {
        if line.contains(&search) {
            result.push((idx + 1, line));
        }
    }
    result
}

pub fn isearch<'a>(search: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let search = search.to_lowercase();
    let mut result: Vec<(usize, &'a str)> = Vec::new();
    for (idx, line) in contents.lines().into_iter().enumerate() {
        if line.to_lowercase().contains(&search) {
            result.push((idx + 1, line));
        }
    }
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn one_result() {
        let query = "rust";
        let contents = "\
Rust is a very fast
language and Rustaceans love it.
Rustaceans are people who love rust";

        assert_eq!(
            vec![(3, "Rustaceans are people who love rust")],
            super::search(query, contents)
        )
    }

    #[test]
    fn two_result() {
        let query = "Rusta";
        let contents = "\
Rust is a very fast
language and Rustaceans love it.
Rustaceans are people who love rust";

        assert_eq!(
            vec![
                (2, "language and Rustaceans love it."),
                (3, "Rustaceans are people who love rust"),
            ],
            super::search(query, contents)
        )
    }

    #[test]
    fn zero_result() {
        let query = "Gopher";
        let contents = "\
Rust is a very fast
language and Rustaceans love it.
Rustaceans are people who love rust";

        let expected: Vec<(usize, &str)> = vec![];
        assert_eq!(expected, super::search(query, contents))
    }

    #[test]
    fn ione_result() {
        let query = "FAst";
        let contents = "\
Rust is a very fast
language and Rustaceans love it.
Rustaceans are people who love rust";

        assert_eq!(
            vec![(1, "Rust is a very fast")],
            super::isearch(query, contents)
        )
    }

    #[test]
    fn itwo_result() {
        let query = "LangU";
        let contents = "\
Rust is a very fast
language and Rustaceans love it.
Rustaceans are people who love rust";

        assert_eq!(
            vec![(2, "language and Rustaceans love it.")],
            super::isearch(query, contents)
        )
    }

    #[test]
    fn izero_result() {
        let query = "Gopher";
        let contents = "\
Rust is a very fast
language and Rustaceans love it.
Rustaceans are people who love rust";

        let expected: Vec<(usize, &str)> = vec![];
        assert_eq!(expected, super::isearch(query, contents))
    }
}
