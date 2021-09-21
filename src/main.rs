extern crate git2;

use std::env;
use std::process;

use timesheet::{find_repository_details, read_input, use_existing_repository, Config, Repo};

fn main() {
    let mut input: String = String::new();

    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let path = match config.repository_path {
        Some(arg) => arg,
        None => {
            println!("Initialise Timesheet for current repository? (Y/n)");
            let option = read_input(&mut input);
            use_existing_repository(Some(&option))
        }
    };

    if let 0 = path.len() {
        eprintln!("Error parsing repository path");
        process::exit(1);
    }

    let repo: Repo = find_repository_details(&*path);

    println!("namespace: {:?} path: {:?}", repo.namespace, repo.path);
}
