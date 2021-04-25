use super::store::Store;

use serde_yaml;
use serde::{ser, de};

const DEFAULT_FILE: &str = "todos.yml";

pub struct YamlStore {
    file: String,
}

impl YamlStore {
    pub fn new(source: Option<String>) -> Self {
        let file = source.unwrap_or(DEFAULT_FILE.to_string());

        Self {
            file,
        }
    }

    pub fn read<T: de::DeserializeOwned>(&self) -> Box<T> {
        let data = std::fs::read_to_string(&self.file).unwrap();

        match serde_yaml::from_str(&data) {
            Ok(t) => t,
            Err(e) => panic!("Error parsing file: {}", e),
        }
    }

    pub fn write<T: ser::Serialize>(&self, data: &T) -> () {
        let s = serde_yaml::to_string(data).unwrap();

        std::fs::write(&self.file, s).unwrap()
    }

    pub fn get_source(&self) -> &str {
        &self.file
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn init_with_passed_source() {
    //     let yaml_store: YamlStore = YamlStore::new(Some("test.yml"));
    //     assert_eq!(yaml_store.file, "test.yml");
    // }

    //
    // #[test]
    // fn init_without_passed_source() {
    //     let yaml_store: YamlStore = YamlStore::new(None);
    //     assert_eq!(yaml_store.file, DEFAULT_FILE);
    // }
    //
    // #[test]
    // fn get_source() {
    //     let yaml_store: YamlStore = YamlStore::new(Some("test.yml"));
    //     assert_eq!(yaml_store.get_source(), String::from("test.yml"));
    // }
}
