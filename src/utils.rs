use crate::config;
use crate::repo;

use git2::Repository;
use std::error::Error;
use std::process;

use random_string::generate;

#[derive(Debug)]
pub enum Commands {
    Init,
    Make,
}

impl std::str::FromStr for Commands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "init" | "-i" => Ok(Commands::Init),
            "make" | "-m" => Ok(Commands::Make),
            _ => Err(format!("'{}' is not a valid value for Commands", s)),
        }
    }
}

pub fn generate_random_path() -> String {
    let charset = "0123456789abcdefghijklmnopqrstuvwxyz";
    generate(10, charset)
}

// TODO - should consider using git2 "discover" so that a repository can be suggested
pub fn find_repository_details(path: &str) -> Result<repo::Repo, Box<dyn Error>> {
    let mut name = String::new();
    let mut email = String::new();

    let repository = Repository::open(path)?;
    let path = repository.path();
    let cfg = repository.config()?;

    for entry in &cfg.entries(None).unwrap() {
        let entry = entry.unwrap();
        if entry.name().unwrap() == "user.name" {
            name = String::from(entry.value().unwrap());
        }
        if entry.name().unwrap() == "user.email" {
            email = String::from(entry.value().unwrap());
        }
    }

    Ok(repo::Repo::new(path, name, email, None)?)
}

pub fn run(config: config::Config) {
    // Match the command against an enum of cli commands
    let command: Commands = config.command.parse().unwrap();
    match command {
        Commands::Init => config.initialise().unwrap_or_else(|err| {
            eprintln!("Error parsing configuration: {}", err);
            process::exit(1);
        }),
        Commands::Make => config.make().unwrap_or_else(|err| {
            eprintln!("Error generating timesheet: {}", err);
            process::exit(1);
        }),
    }

    // If command isn't found, go through on boarding process
    // and generate timesheet config file
    config.onboarding();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_repository_details() {}
}
