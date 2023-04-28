use std::collections::vec_deque::VecDeque;
use std::{
    env,
    fs::{self, DirEntry},
    path::Path,
};

pub fn run(config: Config) -> Result<(), &'static str> {
    println!("Results");
    if config.is_dir {
        let files_to_search = populate_files_to_search(&config.query, &config.path);
        println!("{:?}", &files_to_search);
        for file in files_to_search {
            let contents = match fs::read_to_string(file.path()) {
                Ok(val) => val,
                Err(_) => return Err("Cannot read file"),
            };
            if config.case_sensitive {
                let search_results = search(&config.query, &contents);
                if search_results.len() > 0 {
                    println!("{:?}", file.path());
                    for (line_num, line) in search_results {
                        println!("{:?}. {}", line_num, line);
                    }
                }
            } else if !config.case_sensitive {
                let search_results = isearch(&config.query, &contents);
                if search_results.len() > 0 {
                    println!("{:?}", file.file_name());
                    for (line_num, line) in search_results {
                        println!("{:?}. {}", line_num, line);
                    }
                }
            }
        }
    } else if config.case_sensitive {
        let contents = match fs::read_to_string(config.path) {
            Ok(val) => val,
            Err(_) => return Err("Cannot read file"),
        };
        let res = search(&config.query, &contents);
        for (line_num, line) in res {
            println!("{:?}. {}", line_num, line);
        }
    } else if !config.case_sensitive {
        let contents = match fs::read_to_string(config.path) {
            Ok(val) => val,
            Err(_) => return Err("Cannot read file"),
        };
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
    pub is_dir: bool,
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
        let is_dir = match args.next() {
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
            is_dir,
        })
    }

    pub fn path_exists(&self) -> bool {
        Path::new(&self.path).exists()
    }
}

pub fn populate_files_to_search(_: &String, path: &str) -> Vec<DirEntry> {
    let mut queue: VecDeque<DirEntry> = VecDeque::new();
    let mut files_to_search: Vec<DirEntry> = Vec::new();
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        _ => return files_to_search,
    };
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => return files_to_search,
        };

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => return files_to_search,
        };

        if file_type.is_dir() {
            queue.push_back(entry);
        } else if file_type.is_file() {
            files_to_search.push(entry)
        }
    }
    while queue.len() > 0 {
        let entry = queue.pop_front().unwrap();
        let entries = match fs::read_dir(entry.path()) {
            Ok(entries) => entries,
            _ => return files_to_search,
        };
        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => return files_to_search,
            };

            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(_) => return files_to_search,
            };

            if file_type.is_dir() {
                queue.push_back(entry);
            } else if file_type.is_file() {
                files_to_search.push(entry)
            }
        }
    }
    files_to_search
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
