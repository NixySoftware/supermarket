// This file is just here for testing without accidentally publishing credentials

use std::{collections::HashMap, fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub struct Credentials {
    is_loaded: bool,
    values: HashMap<String, Value>,
}

impl Credentials {
    pub fn new() -> Self {
        Credentials {
            is_loaded: false,
            values: HashMap::new(),
        }
    }

    pub fn get<D: DeserializeOwned>(&mut self, key: &str) -> Option<D> {
        if !self.is_loaded {
            self.load();
        }
        if let Some(value) = self.values.get(&key.to_string()) {
            Some(serde_json::from_value::<D>(value.clone()).unwrap())
        } else {
            None
        }
    }

    pub fn set<S: Serialize>(&mut self, key: &str, value: S) {
        self.values
            .insert(key.to_string(), serde_json::to_value(value).unwrap());
        self.save()
    }

    fn load(&mut self) {
        let path = Path::new("./credentials.json");
        if path.exists() {
            if let Ok(contents) = fs::read_to_string(path) {
                if let Ok(values) = serde_json::from_str::<HashMap<String, Value>>(&contents) {
                    self.values = values
                }
            }
        }
        self.is_loaded = true;
    }

    fn save(&self) {
        let path = Path::new("./credentials.json");

        if let Ok(contents) = serde_json::to_string(&self.values) {
            if fs::write(path, contents).is_ok() {
                ()
            }
        }
    }
}
