use homedir::my_home;
use serde::{Deserialize, Serialize};
use std::fs::File;

const CONFIG: &str = ".bp.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub account: String,
    pub queue: String,
}

impl Config {
    pub fn new(account: String, queue: String) -> Self {
        Self { account, queue }
    }

    pub fn read() -> Option<Self> {
        match my_home().unwrap() {
            Some(home) => {
                let path = home.join(CONFIG);
                match File::open(path) {
                    Ok(file) => {
                        let config: Config =
                            serde_json::from_reader(file).expect("failed to parse config JSON");
                        Some(config)
                    }
                    Err(_) => None,
                }
            }
            None => panic!("failed to get home directory"),
        }
    }

    pub fn write(self) {
        match my_home().unwrap() {
            Some(home) => {
                let path = home.join(CONFIG);
                let file = File::create(path).expect("failed to create config file");
                serde_json::to_writer(file, &self).expect("failed to write config JSON");
            }
            None => panic!("failed to get home directory"),
        }
    }
}
