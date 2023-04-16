use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1);
    });

    println!("Searching for {:?}", config.query);
    println!(
        "In file {:?}",
        config.path.to_str().unwrap_or_else(|| {
            println!("Enter valid file path");
            process::exit(1);
        })
    );

    if !config.path_exists() {
        println!("The file does not exist");
    }

    if let Err(e) = minigrep::run(config) {
        println!("{:?}", e);
        process::exit(1);
    }
}
