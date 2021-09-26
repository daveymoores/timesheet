use exitcode;
use regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;

// Creates a new repository struct after being sent data from git2.
// It returns the namespace and path, but also init date for the repo and probably other stuff
// Basically sanitise the data from git2 into something usable
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Repo {
    pub namespace: String,
    pub path: String,
    pub name: String,
    pub email: String,
}

//TODO: get date out of the repository object
impl Repo {
    pub fn new(
        git_filepath: &Path,
        name: String,
        email: String,
        repo_name: Option<String>,
    ) -> Result<Repo, regex::Error> {
        let mut namespace = String::new();
        // Get repo name by finding the name of the root directory
        let path = git_filepath.display().to_string();

        match repo_name {
            Some(arg) => namespace = arg,
            None => {
                let reg = regex::Regex::new(r"(?P<namespace>[^/][\w\d]+)/\.git/")?;
                for cap in reg.captures_iter(&git_filepath.to_str().unwrap()) {
                    namespace = String::from(&cap["namespace"]);
                }
            }
        };

        Ok(Repo {
            namespace,
            path,
            name,
            email,
        })
    }

    pub fn write_config_file(&self, config_path: &String) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&self).unwrap();
        let mut file = File::create(&config_path)?;

        file.write_all(json.as_bytes())?;
        println!(
            "timesheet-gen initialised. Try 'timesheet-gen make' to create your first timesheet."
        );
        process::exit(exitcode::OK);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;

    #[test]
    fn it_creates_a_repo_struct() {
        let repo = match Repository::open(".") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        let mock_repo = Repo {
            namespace: String::from("timesheet"),
            path: String::from("/path/to/timesheet"),
            name: String::from("Tom Jones"),
            email: String::from("sex_bomb@gmail.com"),
        };

        let repo = Repo::new(
            repo.path(),
            "Tom Jones".to_string(),
            "sex_bomb@gmail.com".to_string(),
            None,
        );
        assert_eq!(repo.unwrap().namespace, mock_repo.namespace);
    }
}