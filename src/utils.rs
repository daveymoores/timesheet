use crate::config::{Commands, GetCommand, Initialise, Make};
use crate::repo;

#[cfg(test)]
use crate::mock_repo_dep::MockRepository as Repository;
#[cfg(not(test))]
use git2::Repository;

use std::error::Error;
use std::{io, process};

use random_string::generate;

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

    Ok(repo::Repo::new(
        None,
        path,
        name,
        email,
        "".to_string(),
        "".to_string(),
        "".to_string(),
    )?)
}

pub fn read_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Input not valid");
    input.trim().to_lowercase()
}

pub fn run<T: Make + Initialise + GetCommand>(config: T) {
    // Match the command against an enum of cli commands
    let command: Commands = config.get_command();
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

    // If command isn't found, show help or suggest command somehow
    println!("Command not found. Run 'timesheet-gen help' for list of commands")
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex;
    use std::path::{Path, PathBuf};

    #[test]
    fn it_generates_a_random_string() {
        let random_string = generate_random_path();
        let regex = regex::Regex::new(r"^[a-z0-9]{10}$");
        match regex.unwrap().find(&*random_string) {
            Some(_x) => assert!(true),
            None => panic!("Pattern not matched"),
        }
    }

    #[test]
    fn it_runs_with_make_and_calls_make() {
        pub struct MockConfig {
            pub command: String,
            pub repository_path: Option<String>,
            pub home_path: PathBuf,
        }

        impl Make for MockConfig {
            fn make(&self) -> Result<(), Box<dyn Error>> {
                assert!(true);
                process::exit(exitcode::OK);
            }
        }

        impl Initialise for MockConfig {
            fn initialise(&self) -> Result<(), Box<dyn Error>> {
                panic!("Wrong function called for command");
            }
        }

        impl GetCommand for MockConfig {
            fn get_command(&self) -> Commands {
                Commands::Make
            }
        }

        run(MockConfig {
            command: "make".to_string(),
            repository_path: Option::from("path/to/.git/".to_string()),
            home_path: Default::default(),
        });
    }

    #[test]
    fn it_runs_with_init_and_calls_init() {
        pub struct MockConfig {
            pub command: String,
            pub repository_path: Option<String>,
            pub home_path: PathBuf,
        }

        impl Make for MockConfig {
            fn make(&self) -> Result<(), Box<dyn Error>> {
                panic!("Wrong function called for command");
            }
        }

        impl Initialise for MockConfig {
            fn initialise(&self) -> Result<(), Box<dyn Error>> {
                assert!(true);
                process::exit(exitcode::OK);
            }
        }

        impl GetCommand for MockConfig {
            fn get_command(&self) -> Commands {
                Commands::Init
            }
        }

        run(MockConfig {
            command: "init".to_string(),
            repository_path: Option::from("path/to/.git/".to_string()),
            home_path: Default::default(),
        });
    }

    #[test]
    fn it_find_repository_details() {
        let repo = repo::Repo::new(
            None,
            Path::new("/path/to/.git/"),
            "Tom Jones".to_string(),
            "sex_bomb@gmail.com".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        );
        assert_eq!(
            find_repository_details("/path/to/.git/").unwrap(),
            repo.unwrap()
        );
    }
}
