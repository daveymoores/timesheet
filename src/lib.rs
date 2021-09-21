use git2::Repository;
use regex::{Error, Regex};
use std::fs::File;
use std::io::{Read, Write};
use std::{env, io, io::ErrorKind, process};

use serde::{Deserialize, Serialize};
use serde_json;

const CONFIG_FILE_NAME: &str = ".timesheet-gen.txt";

#[derive(Debug)]
pub enum Commands {
    Init,
}

impl std::str::FromStr for Commands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "init" => Ok(Commands::Init),
            _ => Err(format!("'{}' is not a valid value for Commands", s)),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub command: String,
    pub repository_path: Option<String>,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, io::Error> {
        let argument_length = args.len();

        if argument_length <= 1 {
            let not_enough_arguments =
                io::Error::new(ErrorKind::InvalidInput, format!("Not enough arguments"));
            return Err(not_enough_arguments);
        }

        args.next();
        let command = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    format!("Didn't get a query 'command'"),
                ))
            }
        };

        let repository_path = args.next();

        Ok(Config {
            command,
            repository_path,
        })
    }
}

// Creates a new repository struct after being sent data from git2.
// It returns the namespace and path, but also init date for the repo and probably other stuff
// Basically sanitise the data from git2 into something usable
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Repo {
    pub namespace: String,
    pub path: String,
}

//TODO: get date out of the repository object
impl Repo {
    pub fn new(repository: Repository) -> Result<Repo, Error> {
        let mut namespace = String::new();
        // Get repo name by finding the name of the root directory
        let path = repository.path().display().to_string();
        let regex = Regex::new(r"(?P<namespace>[^/][\w\d]+)/\.git/")?;
        for cap in regex.captures_iter(repository.path().to_str().unwrap()) {
            namespace = String::from(&cap["namespace"]);
        }

        Ok(Repo { namespace, path })
    }
}

pub fn read_input(input: &mut String) -> String {
    io::stdin().read_line(input).expect("Input not valid");
    input.trim().to_lowercase()
}

pub fn use_existing_repository(option: Option<&str>) -> String {
    match option {
        Some("") => String::from("."),
        Some("y") => String::from("."),
        Some("n") => {
            let mut input = String::new();
            println!("Please give a path to the repository you would like to use:");
            read_input(&mut input)
        }
        _ => {
            println!("Invalid input. Falling back to current directory.");
            ".".to_string()
        }
    }
}

pub fn check_for_existing_config_file() {
    let path = get_filepath();

    match File::open(&path) {
        Err(_) => println!("This looks like the first time you're running Timesheet."),
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("couldn't read {}: {}", path, why),
                Ok(_) => {
                    let mut input = String::new();
                    println!("We've found an existing configuration at:\n{}\n", path);
                    println!("Would you like to use this configuration? (Y/n)");
                    read_input(&mut input);
                    process::exit(1);
                }
            }
        }
    };
}

// TODO - should consider using git2 "discover" so that a repository
// can be suggested if the user isn't in the correct repository
fn find_repository_details(path: &str) -> Repo {
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    Repo::new(repo).unwrap_or_else(|err| {
        eprintln!("Repo not found: {}", err);
        process::exit(1);
    })
}

fn get_filepath() -> String {
    let home = match dirs::home_dir() {
        Some(arg) => arg,
        None => panic!("Home directory not found"),
    };
    let home_string = home.to_str();
    home_string.unwrap().to_owned() + "/" + CONFIG_FILE_NAME
}

pub fn create_user_config(path: &str) {
    let repo: Repo = find_repository_details(&*path);
    let json = serde_json::to_string(&repo).unwrap();
    let path = get_filepath();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path, why),
        Ok(file) => file,
    };

    match file.write_all(json.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path, why),
        Ok(_) => println!("successfully wrote to {}", path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_repo_struct() {
        let repo = match Repository::open(".") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        let mock_repo = Repo {
            namespace: String::from("timesheet"),
            path: String::from("/path/to/timesheet"),
        };

        let repo = Repo::new(repo);
        assert_eq!(repo.namespace, mock_repo.namespace);
    }
}
