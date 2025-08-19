use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub data: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn load_from_file(path: &str) -> Self {
        match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Database::new()),
            Err(_) => Database::new(),
        }
    }

    pub fn save_to_file(&self, path: &str) {
        if let Ok(json) = serde_json::to_string_pretty(&self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    pub fn update(&mut self, key: &str, value: &str) {
        if self.data.contains_key(key) {
            self.data.insert(key.to_string(), value.to_string());
        }
    }

    pub fn delete(&mut self, key: &str) {
        if self.data.contains_key(key) {
            self.data.remove(key);
        }
    }

    pub fn get(&mut self, key: &str) {
        if self.data.contains_key(key) {
            println!("{:?}", self.data.get(key).unwrap())
        }
    }
}
