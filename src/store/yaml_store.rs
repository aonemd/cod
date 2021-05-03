use super::store::Store;

use std::fs::File;
use std::path::Path;

use serde_yaml;
use serde::{ser, de};

pub struct YamlStore {
    file: String,
}

impl YamlStore {
    pub fn new(source: Option<String>) -> Self {
        let file = source.unwrap_or(Self::default_source());

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

    fn default_source() -> String {
        format!("{}/.todos.yml", std::env::var("HOME").unwrap())
    }
}

#[cfg(test)]
mod yaml_store_tests {
    use super::*;

    #[test]
    fn init_with_passed_source() {
        let yaml_store: YamlStore = YamlStore::new(Some("test.yml".to_string()));
        assert_eq!(yaml_store.file, "test.yml");
    }


    #[test]
    fn init_without_passed_source() {
        let yaml_store: YamlStore = YamlStore::new(None);
        assert_eq!(yaml_store.file, YamlStore::default_source());
    }

    #[test]
    fn get_source() {
        let yaml_store: YamlStore = YamlStore::new(Some("test.yml".to_string()));
        assert_eq!(yaml_store.get_source(), String::from("test.yml"));
    }
}
