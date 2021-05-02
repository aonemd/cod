use super::item::Item;
use super::todo_serialized::TodoSerialized;
use super::todo_presenter::TodoPresenter;

use chrono::{NaiveDate, Local};

#[derive(Debug)]
pub struct Todo {
    pub items: Vec<Item>,
    pub last_id: u32,
}

impl From<TodoSerialized> for Todo {
    fn from(todo_serialized: TodoSerialized) -> Self {
        let items = todo_serialized.internal_map().values().cloned().flatten().collect();
        let last_id = largest_id(&items);

        Self {
            items,
            last_id,
        }
    }
}

impl Todo {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            last_id: 0,
        }
    }

    pub fn list(&self) -> () {
        let presenter = TodoPresenter::new(self);

        presenter.present();
    }

    pub fn add(&mut self, desc: Option<String>, date: Option<NaiveDate>, tags: Option<Vec<String>>) {
        let next_id = self.get_next_id();
        let desc = desc.expect("Item description cannot be empty!");
        let date = date.unwrap_or(Local::today().naive_local());
        let tags = tags.unwrap_or(vec![]);
        let completed = false;
        let new_item = Item::new(next_id, desc, date, tags, completed);

        self.items.push(new_item);
    }

    fn get_next_id(&self) -> u32 {
        self.last_id() + 1
    }

    fn last_id(&self) -> u32 {
        // let last_id = match self.items.len() {
        //     0 => 0,
        //     _ => {
        //         let mut sorted: Vec<u32> = self.items.iter().map(|item| item.id).collect();
        //         sorted.sort_by(|a, b| a.cmp(&b));
        //         *sorted.last().unwrap()
        //     }
        // };
        //
        // last_id
        largest_id(&self.items)
    }
}

fn largest_id(items: &Vec<Item>) -> u32 {
    let last_id = match items.len() {
        0 => 0,
        _ => {
            let mut sorted: Vec<u32> = items.iter().map(|item| item.id).collect();
            sorted.sort_by(|a, b| a.cmp(&b));
            *sorted.last().unwrap()
        }
    };

    last_id
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Local};

    #[test]
    fn add_increments_items_length() {
        let mut todo = Todo::new();

        todo.add(Some(String::from("Hello, world!")), Some(Local::today().naive_local()), Some(vec![String::from("tag1")]));

        assert_eq!(todo.items.len(), 1);
    }

    #[test]
    fn add_increments_last_item_id() {
        let mut todo = Todo::new();
        todo.add(Some(String::from("Hello, world!")), Some(Local::today().naive_local()), Some(vec![String::from("tag1")]));
        todo.add(Some(String::from("Hello, world war II!")), Some(Local::today().naive_local()), Some(vec![String::from("tag1")]));

        todo.add(Some(String::from("Hello, world war III!")), Some(Local::today().naive_local()), Some(vec![String::from("tag1")]));

        assert_eq!(todo.items.last().unwrap().id, 3);
        assert_eq!(todo.items.len(), 3);
    }
}
