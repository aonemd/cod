use super::store::Store;

use std::fs::File;
use std::path::Path;

use serde_yaml;
use serde::{ser, de};

const DEFAULT_FILE: &str = "todos.yml";

pub struct YamlStore {
    file: String,
}

impl YamlStore {
    pub fn new(source: Option<String>) -> Self {
        let file = source.unwrap_or(DEFAULT_FILE.to_string());

        Self::create_file_if_not_exist(&file);

        Self {
            file,
        }
    }

    pub fn read<T: de::DeserializeOwned>(&self) -> Result<Box<T>, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(&self.file).unwrap();

        return Ok(serde_yaml::from_str(&data)?);
    }

    pub fn write<T: ser::Serialize>(&self, data: &T) -> () {
        let s = serde_yaml::to_string(data).unwrap();

        std::fs::write(&self.file, s).unwrap()
    }

    pub fn get_source(&self) -> &str {
        &self.file
    }

    fn create_file_if_not_exist(file: &str) {
        if !Path::new(file).exists()  {
            File::create(file).unwrap();
        }
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
