use crate::constants;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub repository: String,
    pub files: Vec<String>,
}

pub fn get_config() -> Config {
    let config_file = File::open(constants::CONFIG_FILE).expect("error while opening config file");
    let config: Config =
        serde_json::from_reader(config_file).expect("error while reading config file");

    config
}

#[cfg(test)]
mod tests {
    use crate::etc::get_config;

    #[test]
    fn test_get_config() {
        let config = get_config();
        println!("Repo: {}", config.repository);
        for file in config.files.iter() {
            println!("File: {}", file);
        }
    }
}
