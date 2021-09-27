extern crate git2;
use std::env;
use std::process;

mod config;
mod db;
mod mock_repo_dep;
mod repo;
mod utils;

fn main() {
    // Construct Config struct with repo path and commands
    let config: config::Config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    utils::run(config);
}
