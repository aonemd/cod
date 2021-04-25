use std::collections::HashMap;

use chrono::{NaiveDate};
use serde::{Serialize, Deserialize};


use super::item::Item;
use super::todo::Todo;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoSerialized(HashMap<NaiveDate, Vec<Item>>);

impl From<&Todo> for TodoSerialized {
    fn from(todo: &Todo) -> Self {
        let mut groups: HashMap<NaiveDate, Vec<Item>> = HashMap::new();
        for _item in &todo.items {
            groups.entry(_item.date).or_insert(vec![]).push(_item.clone())
        }

        Self(groups)
    }
}

impl TodoSerialized {
    pub fn internal_map(&self) -> &HashMap<NaiveDate, Vec<Item>> {
        &self.0
    }
}
