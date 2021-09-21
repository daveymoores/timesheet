use git2::Repository;
use regex::Regex;
use std::{io, io::ErrorKind};

#[derive(PartialEq, Debug)]
pub struct Config {
    pub command: String,
    pub repository_path: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, io::Error> {
        let argument_length = args.len();

        if argument_length <= 1 {
            let not_enough_arguments =
                io::Error::new(ErrorKind::InvalidInput, format!("Not enough arguments"));
            return Err(not_enough_arguments);
        }

        let command = args[1].clone();
        let repository_path = match argument_length {
            3 => Some(args[2].clone()),
            _ => None,
        };

        Ok(Config {
            command,
            repository_path,
        })
    }
}

// Creates a new repository struct after being sent data from git2.
// It returns the namespace and path, but also init date for the repo and probably other stuff
// Basically sanitise the data from git2 into something usable
#[derive(PartialEq, Debug)]
pub struct Repo {
    pub namespace: String,
    pub path: String,
    //pub start_date: DateTime<Utc>,
}

//TODO: get date out of the repository object
impl Repo {
    pub fn new(repository: Repository) -> Repo {
        let mut namespace = String::new();
        // Get repo name by finding the name of the root directory
        let path = repository.path().display().to_string();
        let regex = Regex::new(r"([^/][\w\d]+)/\.git/").unwrap();
        for cap in regex.captures_iter(repository.path().to_str().unwrap()) {
            namespace = String::from(&cap[1]);
        }
        Repo { namespace, path }
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

// TODO - should consider using git2 "discover" so that a repository
// can be suggested if the user isn't in the correct repository
pub fn find_repository_details(path: &str) -> Repo {
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    Repo::new(repo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_config_struct_with_path_option() {
        let args: Vec<String> = vec![
            String::from("target/debug/timesheet"),
            String::from("init"),
            String::from("/path/to/somewhere"),
        ];
        let config = Config::new(&args);
        let values = config.as_ref().unwrap();

        let mock_config = Config {
            command: String::from("init"),
            repository_path: Option::from(String::from("/path/to/somewhere")),
        };

        assert_eq!(values, &mock_config);
    }

    #[test]
    fn it_creates_a_config_struct_without_a_path_option() {
        let args: Vec<String> = vec![String::from("target/debug/timesheet"), String::from("init")];
        let config = Config::new(&args);
        let values = config.as_ref().unwrap();

        let mock_config = Config {
            command: String::from("init"),
            repository_path: None,
        };

        assert_eq!(values, &mock_config);
    }

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
