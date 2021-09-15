extern crate git2;

use git2::Repository;
use std::io;

fn is_existing_repository(option: Option<&str>) -> String {
    match option {
        Some("yes") => String::from("."),
        Some("no") => {
            let mut input2 = String::new();
            println!("Please give a path to the repository you would like to use:");
            io::stdin().read_line(&mut input2).expect("Input not valid");
            input2.trim().to_lowercase()
        },
        _ => {
            println!("Invalid input. Falling back to current directory.");
            ".".to_string()
        },
    }
}

fn main() {
    println!("Would you like to use the existing repository (Yes/No)?");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Input not valid");

    let option = input.trim().to_lowercase();
    let path = is_existing_repository(Some(&option));

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    println!("{} state={:?}", repo.path().display(), repo.state());
}