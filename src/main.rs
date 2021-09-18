extern crate git2;

use std::env;
use std::process;

use timesheet::{find_repository_details, read_input, use_existing_repository, Config, Repo};

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
        let option = read_input(&mut input);
        path = use_existing_repository(Some(&option));
    }

    if let 0 = path.len() {
        eprintln!("Error parsing repository path");
        process::exit(1);
    }

    let repo: Repo = find_repository_details(&*path);

    println!("namespace: {:?} path: {:?}", repo.namespace, repo.path);
}
