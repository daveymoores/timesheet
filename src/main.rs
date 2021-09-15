extern crate git2;

use git2::Repository;
use std::io;

fn main() {
    println!("Would you like to use the existing repository (Yes/No)?");
    let mut input = String::new();
    let mut input2 = String::new();
    let path;

    io::stdin().read_line(&mut input).expect("Input not valid");

    let option = input.trim().to_lowercase();
    if option == "yes" {
        path = String::from(".");
    } else  {
        println!("Please give a path to the repository you would like to use:");
        io::stdin().read_line(&mut input2).expect("Input not valid");
        path =   input2.trim().to_lowercase();
    }

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    println!("{} state={:?}", repo.path().display(), repo.state());
}