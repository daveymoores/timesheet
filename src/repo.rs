use crate::utils;
use exitcode;
use regex;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
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
    pub client_name: String,
    pub contact_person: String,
    pub address: String,
    pub timesheet: Map<String, Value>,
}

//TODO: get date out of the repository object
impl Repo {
    pub fn new(
        repo_name: Option<String>,
        git_filepath: &Path,
        name: String,
        email: String,
        client_name: String,
        contact_person: String,
        address: String,
        timesheet: Map<String, Value>,
    ) -> Result<Repo, regex::Error> {
        let mut namespace = String::new();
        // Get repo name by finding the name of the root directory
        let path = git_filepath.display().to_string();

        // TODO this will fail at runtime if the git path is incorrect
        match repo_name {
            Some(arg) => namespace = arg,
            None => {
                let reg = regex::Regex::new(r"(?P<namespace>[^/][\w\d]+)/\.git/")?;
                if let Some(cap) = reg
                    .captures(&git_filepath.to_str().unwrap())
                    .unwrap()
                    .name("namespace")
                {
                    // parse into String
                    namespace = (&cap.as_str()).parse().unwrap();
                }
            }
        };

        Ok(Repo {
            namespace,
            path,
            name,
            email,
            client_name,
            contact_person,
            address,
            timesheet,
        })
    }

    pub fn prompt_for_client_details(&mut self) -> &Repo {
        println!("Would you like to add a client for this repository? Y/n");
        self.use_client_option();
        self
    }

    fn use_client_option(&mut self) {
        let input = utils::read_input().to_lowercase();
        let option = Option::from(&*input);
        match option {
            Some("") | Some("y") => self.input_client_option(),
            Some("n") => {}
            _ => {
                println!("Invalid input.");
                process::exit(exitcode::DATAERR);
            }
        };
    }

    fn input_client_option(&mut self) {
        println!("Client name:");
        let client_name = utils::read_input();
        self.client_name = String::from(client_name);

        println!("Client contact person name:");
        let contact_person = utils::read_input();
        self.contact_person = String::from(contact_person);

        println!("Address (comma seperated):");
        let address = utils::read_input();
        let regex = Regex::new(r",\s*").unwrap();
        let result = regex.replace_all(&address, ",\n");
        self.address = String::from(result);
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
            client_name: "".to_string(),
            contact_person: "".to_string(),
            address: "".to_string(),
            timesheet: Map::new(),
        };

        let repo = Repo::new(
            None,
            repo.path(),
            "Tom Jones".to_string(),
            "sex_bomb@gmail.com".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            Map::new(),
        );
        assert_eq!(repo.unwrap().namespace, mock_repo.namespace);
    }
}
