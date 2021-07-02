use std::env;

pub struct Config {
    pub todoist_token: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        let todoist_token = Self::parse_env_var_into_option("COD_TODOIST_TOKEN");

        Self { todoist_token }
    }

    fn parse_env_var_into_option(env_var: &str) -> Option<String> {
        match env::var("COD_TODOIST_TOKEN") {
            Ok(token) => Some(token),
            _ => None,
        }
    }
}
