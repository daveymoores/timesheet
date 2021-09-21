extern crate git2;
use std::env;
use std::process;

use timesheet::{
    check_for_existing_config_file, create_user_config, read_input, use_current_repository,
    Commands, Config,
};

fn main() {
    let mut input: String = String::new();

    // Construct Config struct with repo path and commands
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Match the command against an enum of cli commands
    let command: Commands = config.command.parse().unwrap();
    match command {
        Commands::Init => check_for_existing_config_file().unwrap_or_else(|err| {
            eprintln!("Error parsing configuration: {}", err);
            process::exit(1);
        }),
    }

    // If command isn't found, check go through on boarding process
    // and generate timesheet config file
    let path = match config.repository_path {
        Some(arg) => arg,
        None => {
            println!("Initialise timesheet-gen for current repository? (Y/n)");
            let option = read_input(&mut input);
            use_current_repository(Some(&option))
        }
    };

    if let 0 = path.len() {
        eprintln!("Error parsing repository path");
        process::exit(1);
    }

    create_user_config(&*path);
}
