use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;


#[derive(Debug, Serialize, Deserialize)]
pub struct Processor {
    pub cores: u32,
    pub clock_speed: u32,
    pub scheduler_algorithm: String,
    pub preemption_enabled: bool,
}


impl Processor {
    pub fn new(path: Option<&PathBuf>) -> Self {
        match path {
            Some(file_path) => match fs::read_to_string(file_path) {
                Ok(content) => toml::from_str(&content).unwrap_or_else(|_| Processor::default()),
                Err(_) => Self::default(),
            },
            None => Self::default(),
        }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Processor {
            cores: 1,
            clock_speed: 1000,
            scheduler_algorithm: "round_robin".to_string(),
            preemption_enabled: true,
        }
    }
}
