extern crate git2;

use git2::Repository;
use std::process;
use std::{env, io};

use timesheet::Config;

fn use_existing_repository(option: Option<&str>) -> String {
    match option {
        Some("") => String::from("."),
        Some("y") => String::from("."),
        Some("n") => {
            let mut input2 = String::new();
            println!("Please give a path to the repository you would like to use:");
            io::stdin().read_line(&mut input2).expect("Input not valid");
            input2.trim().to_lowercase()
        }
        _ => {
            println!("Invalid input. Falling back to current directory.");
            ".".to_string()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: String = String::new();
    let mut path = String::new();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let None = config.repository_path {
        println!("Initialise Timesheet for current repository? (Y/n)");
        io::stdin().read_line(&mut input).expect("Input not valid");
        let option = input.trim().to_lowercase();
        path = use_existing_repository(Some(&option));
    }

    if let 0 = path.len() {
        eprintln!("Error parsing repository path");
        process::exit(1);
    }

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    println!("{} state={:?}", repo.path().display(), repo.state());
}
