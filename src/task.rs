use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct TasksConfig {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub priority: u32,
    pub execution_time: u32,
    pub period: u32,
}


impl TasksConfig {
    pub fn new(path: Option<&PathBuf>) -> Self {
        println!("Loading tasks from {:?}", path.unwrap());
        match path {
            Some(file_path) => match fs::read_to_string(file_path) {
                Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
                    println!("Error reading tasks file: {}", e);
                    Self::default()
                }),
                Err(e) => {
                    println!("Error reading tasks file: {}", e);
                    Self::default()
                },
            },
            None => {
                println!("No tasks file provided, using default configuration");
                Self::default()
            },
        }
    }
}

impl Default for TasksConfig {
    fn default() -> Self {
        Self {
            tasks: vec![
                Task {
                    id: 1,
                    name: "Task1".to_string(),
                    priority: 1,
                    execution_time: 10,
                    period: 100,
                },
                Task {
                    id: 2,
                    name: "Task2".to_string(),
                    priority: 2,
                    execution_time: 20,
                    period: 200,
                },
            ],
        }
    }
}
