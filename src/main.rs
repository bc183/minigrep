use minigrep::Config;
use std::{env, process};

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1);
    });

    println!("Searching for {:?}", config.query);
    println!("In {:?}", config.path);

    if !config.path_exists() {
        println!("The file does not exist");
        process::exit(1);
    }

    if let Err(e) = minigrep::run(config) {
        println!("{:?}", e);
        process::exit(1);
    }
}
