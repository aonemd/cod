use super::item::Item;
use super::todo_serialized::TodoSerialized;

use chrono::{NaiveDate};

#[derive(Debug)]
pub struct Todo {
    pub items: Vec<Item>,
}

impl From<TodoSerialized> for Todo {
    fn from(todo_serialized: TodoSerialized) -> Self {
        let items = todo_serialized.internal_map().values().cloned().flatten().collect();

        Self {
            items,
        }
    }
}

impl Todo {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, desc: String, date: NaiveDate, tags: Vec<String>) {
        let next_id = self.get_next_id();
        let new_item = Item::new(next_id, desc, date, tags);
        self.items.push(new_item);
    }

    fn get_next_id(&self) -> u32 {
        self.last_id() + 1
    }

    fn last_id(&self) -> u32 {
        match self.items.len() {
            0 => 0,
            _ => {
                let mut sorted: Vec<u32> = self.items.iter().map(|item| item.id).collect();
                sorted.sort_by(|a, b| a.cmp(&b));
                *sorted.last().unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Local};

    #[test]
    fn add_increments_items_length() {
        let mut todo = Todo::new();

        todo.add(String::from("Hello, world!"), Local::today().naive_local(), vec![String::from("tag1")]);

        assert_eq!(todo.items.len(), 1);
    }

    #[test]
    fn add_increments_last_item_id() {
        let mut todo = Todo::new();
        todo.add(String::from("Hello, world!"), Local::today().naive_local(), vec![String::from("tag1")]);
        todo.add(String::from("Hello, world war II!"), Local::today().naive_local(), vec![String::from("tag1")]);

        todo.add(String::from("Hello, world war III!"), Local::today().naive_local(), vec![String::from("tag1")]);

        assert_eq!(todo.items.last().unwrap().id, 3);
        assert_eq!(todo.items.len(), 3);
    }
}
