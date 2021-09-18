use std::{io, io::ErrorKind};

#[derive(PartialEq, Debug)]
pub struct Config {
    pub command: String,
    pub repository_path: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, io::Error> {
        let argument_length = args.len();

        if argument_length < 1 {
            let not_enough_arguments =
                io::Error::new(ErrorKind::InvalidInput, format!("not enough arguments"));
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
    #[ignore]
    fn it_creates_a_config_struct_with_arguments() {}

    #[test]
    #[ignore]
    fn it_runs_and_creates_a_repository_struct() {}

    #[test]
    #[ignore]
    fn it_fails_finding_repository_and_returns_error() {}
}
