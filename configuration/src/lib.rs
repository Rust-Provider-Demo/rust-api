use std::{collections::HashMap, env};

use dotenvy;

pub trait GetEnvVar {
    fn get_var(&self, key: &str) -> Option<&String>;
}

pub struct Config {
    vars: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        let configured_env = env::var("env").unwrap_or(String::from("ci"));

        dotenvy::from_filename(format!(".{configured_env}.env")).ok();

        Config {
            vars: env::vars().into_iter().collect(),
        }
    }
}

impl GetEnvVar for Config {
    fn get_var(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }
}

#[cfg(test)]
mod config_test {

    use std::env::VarError;

    use super::*;

    #[test]
    fn should_read_ci_config() {
        assert_eq!(env::var("env").unwrap(), "ci");
        let config = Config::new();

        assert_eq!(config.get_var("env").unwrap(), &String::from("ci"));
        assert_eq!(
            config.get_var("db_username").unwrap(),
            &String::from("ci-username")
        );
        assert_eq!(
            config.get_var("db_password").unwrap(),
            &String::from("ci-password")
        );
        assert_eq!(
            config.get_var("db_connection").unwrap(),
            &String::from("placeholder")
        );
    }

    #[test]
    fn should_read_ci_config_when_no_env() {
        env::remove_var("env");
        assert_eq!(env::var("env"), Err(VarError::NotPresent));
        let config = Config::new();

        assert_eq!(config.get_var("env").unwrap(), &String::from("ci"));
        assert_eq!(
            config.get_var("db_username").unwrap(),
            &String::from("ci-username")
        );
        assert_eq!(
            config.get_var("db_password").unwrap(),
            &String::from("ci-password")
        );
        assert_eq!(
            config.get_var("db_connection").unwrap(),
            &String::from("placeholder")
        );
    }
}
