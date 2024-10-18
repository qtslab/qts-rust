use crate::types::Problem;

use serde::Deserialize;
use std::fs;
use toml;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub problem: Problem,
    pub algorithm: Algorithm,
    pub test: Test,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Algorithm {
    pub max_gen: i32,
    pub n_neighbors: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Test {
    pub count: i32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            problem: Problem::default(),
            algorithm: Algorithm {
                max_gen: 1000,
                n_neighbors: 10,
            },
            test: Test { count: 100 },
        }
    }
}

impl Config {
    pub fn get() -> Result<Config, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string("config/config.toml")?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}
