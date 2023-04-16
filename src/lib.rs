use std::{env, error::Error, fs, path::Path};

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

pub struct Config {
    pub path: String,
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let path = match args.next() {
            Some(p) => p,
            None => return Err("Path must be specified"),
        };
        let query = match args.next() {
            Some(q) => q,
            None => return Err("Query must be specified"),
        };
        let case_sensitive = match args.next() {
            Some(q) => match q.as_str() {
                "true" => true,
                "false" => false,
                _ => false,
            },
            None => false,
        };
        Ok(Config {
            path,
            query,
            case_sensitive,
        })
    }

    pub fn path_exists(&self) -> bool {
        Path::new(&self.path).exists()
    }
}

pub fn search<'a>(search: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .into_iter()
        .enumerate()
        .filter(|(_, line)| line.contains(&search))
        .map(|(idx, line)| (idx + 1, line))
        .collect()
}

pub fn isearch<'a>(search: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let search = search.to_lowercase();
    contents
        .lines()
        .into_iter()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&search))
        .map(|(idx, line)| (idx + 1, line))
        .collect()
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
