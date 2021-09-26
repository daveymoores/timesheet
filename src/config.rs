extern crate bson;
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::IndexModel;
use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::PathBuf;
use std::{env, io, process};
use tokio;

use crate::db;
use crate::repo;

use crate::db::Db;
use chrono::Utc;
use git2::Repository;
use std::time::Duration;

const CONFIG_FILE_NAME: &str = ".timesheet-gen.txt";

#[derive(PartialEq, Debug)]
pub struct Config {
    pub command: String,
    pub repository_path: Option<String>,
    pub home_path: PathBuf,
}

// Creates a struct with commands and path data
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

        let home_path = match dirs::home_dir() {
            Some(dir) => dir,
            None => panic!("Home directory not found"),
        };

        Ok(Config {
            command,
            repository_path,
            home_path,
        })
    }

    fn get_filepath(&self) -> String {
        let home_string = self.home_path.to_str();
        home_string.unwrap().to_owned() + "/" + CONFIG_FILE_NAME
    }

    fn find_user_data(&self) -> Result<repo::Repo, Box<dyn Error>> {
        let config_path = self.get_filepath();
        let mut buffer = String::new();

        match File::open(&config_path) {
            Ok(mut file) => {
                file.read_to_string(&mut buffer)?;
            }
            Err(_) => {
                println!("This looks like the first time you're running timesheet-gen");
                self.onboarding();
            }
        };

        let config_details: repo::Repo = serde_json::from_str(&*buffer)?;
        let repository = Repository::open(config_details.path)?;
        let path = repository.path();
        let repo = repo::Repo::new(
            path,
            config_details.name,
            config_details.email,
            Some(config_details.namespace),
        )?;

        Ok(repo)
    }

    pub fn initialise(&self) -> Result<(), Box<dyn Error>> {
        let repo = self.find_user_data()?;
        // show the user the contents of the config file
        // and prompt as to whether this file should be used
        self.prompt_for_config_use(repo);
        Ok(())
    }

    // TODO allow the user to edit these values
    fn create_user_config(&self, path: &str, config_path: &String) -> Result<(), Box<dyn Error>> {
        let repo: repo::Repo =
            crate::utils::find_repository_details(&*path).unwrap_or_else(|err| {
                eprintln!("Couldn't find repository details: {}", err);
                process::exit(1);
            });

        repo.write_config_file(&config_path).unwrap_or_else(|err| {
            eprintln!("Couldn't write to configuration file: {}", err);
            process::exit(1);
        });

        Ok(())
    }

    pub fn onboarding(&self) {
        let config_path = self.get_filepath();
        let path;

        match &self.repository_path {
            Some(arg) => path = String::from(arg),
            None => {
                println!("Initialise timesheet-gen for current repository? (Y/n)");
                path = String::from(&self.use_current_repository());
            }
        };

        if let 0 = path.len() {
            eprintln!("Error parsing repository path");
            process::exit(1);
        }

        self.create_user_config(&*path, &config_path)
            .unwrap_or_else(|err| {
                eprintln!("Error creating user configuration: {}", err);
                process::exit(1);
            });
    }

    fn prompt_for_config_use(&self, repo: repo::Repo) {
        let config_path = self.get_filepath();

        println!(
            "timesheet-gen has found an existing configuration at:\n{}\n\
            -------------------------------------------\n\
            Name: {}\n\
            Email: {}\n\
            Project: {}\n\
            Git path: {}\n\
            -------------------------------------------\n\
            Would you like to use this configuration? (Y/n)",
            config_path, repo.name, repo.email, repo.namespace, repo.path
        );

        let option = self.read_input();
        self.use_existing_configuration(Some(&option));
        process::exit(1);
    }

    fn use_existing_configuration(&self, option: Option<&str>) -> String {
        match option {
            Some("") | Some("y") => String::new(),
            Some("n") => String::new(),
            _ => {
                println!("Invalid input. Exiting.");
                process::exit(1);
            }
        }
    }

    fn use_current_repository(&self) -> String {
        let input = self.read_input();
        let option = Option::from(&*input);
        match option {
            Some("") | Some("y") => String::from("."),
            Some("n") => {
                println!("Please give a path to the repository you would like to use:");
                self.read_input()
            }
            _ => {
                println!("Invalid input. Falling back to current directory.");
                ".".to_string()
            }
        }
    }

    fn read_input(&self) -> String {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Input not valid");
        input.trim().to_lowercase()
    }

    #[tokio::main]
    pub async fn make(&self) -> Result<(), Box<dyn Error>> {
        println!("Generating timesheet...");
        let user_data: repo::Repo = self.find_user_data()?;
        // set environment vars
        // connect to mongodb
        // generate random string to use as path
        // check for existing random string. If it exists, generate another
        // push config file as json into storage
        // set TTF in mongodb and trash this after 30 minutes
        // get back random string and create path
        // output path to user - e.g https://timesheet-gen.io/jh57y84hk

        let db: Db = db::Db::new().await?;
        let collection = db
            .client
            .database("timesheet-gen")
            .collection("timesheet-temp-paths");

        let random_path = db.generate_random_path(&collection).await?;

        let timesheet = doc! {
            "creation_date": Utc::now(),
            "random_path": &random_path,
            "name" : user_data.name,
            "email" : user_data.email,
            "namespace" : user_data.namespace,
            "path" : user_data.path,
        };

        let index_model = IndexModel::builder()
            .keys(doc! {"creation_date": 1})
            .options(
                IndexOptions::builder()
                    .expire_after(Duration::new(180, 0))
                    .build(),
            )
            .build();

        &collection.create_index(index_model, None);
        collection.insert_one(timesheet.clone(), None).await?;

        println!(
            "Timesheet now available for 30 minutes @ https://timesheet-gen.io/{}",
            &random_path
        );

        process::exit(exitcode::OK);
    }
}
