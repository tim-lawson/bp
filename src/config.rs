use homedir::my_home;
use serde::{Deserialize, Serialize};
use std::fs::File;

use crate::gpu_type::GpuType;

const CONFIG: &str = ".bp.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: Option<String>,
    pub queue: Option<String>,
    pub gpus: Option<u8>,
    pub gpu_types: Option<Vec<GpuType>>,
    pub hours: Option<u8>,
}

impl Config {
    pub fn new(
        project: Option<String>,
        queue: Option<String>,
        gpus: Option<u8>,
        gpu_types: Option<Vec<GpuType>>,
        hours: Option<u8>,
    ) -> Self {
        Self {
            project,
            queue,
            gpus,
            gpu_types,
            hours,
        }
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
