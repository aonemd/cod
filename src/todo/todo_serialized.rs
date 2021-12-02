use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::item::Item;
use super::todo::Todo;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoSerialized(HashMap<NaiveDateTime, Vec<Item>>);

impl Default for TodoSerialized {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl From<&Todo> for TodoSerialized {
    fn from(todo: &Todo) -> Self {
        let mut groups: HashMap<NaiveDateTime, Vec<Item>> = HashMap::new();
        for _item in &todo.items {
            groups
                .entry(_item.date)
                .or_insert(vec![])
                .push(_item.clone())
        }

        Self(groups)
    }
}

impl TodoSerialized {
    pub fn internal_map(&self) -> &HashMap<NaiveDateTime, Vec<Item>> {
        &self.0
    }
}
