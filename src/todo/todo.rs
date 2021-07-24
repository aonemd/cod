use super::item::Item;
use super::item_source::ItemSource;
use super::todo_presenter::TodoPresenter;
use super::todo_serialized::TodoSerialized;

use chrono::{Local, NaiveDate};

#[derive(Debug)]
pub struct Todo {
    pub items: Vec<Item>,
    pub last_id: u32,
}

impl From<TodoSerialized> for Todo {
    fn from(todo_serialized: TodoSerialized) -> Self {
        let items = todo_serialized
            .internal_map()
            .values()
            .cloned()
            .flatten()
            .collect();
        let last_id = Self::get_largest_id(&items);

        Self { items, last_id }
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

    pub fn edit_or_add(
        &mut self,
        desc: Option<String>,
        date: Option<NaiveDate>,
        tags: Option<Vec<String>>,
        uid: Option<i64>,
        item_source: Option<ItemSource>,
    ) -> () {
        if let Some(original_item) = self.items.iter().find(|i| i.uid == uid.unwrap()) {
            self.edit(original_item.id, desc, date, tags);
        } else {
            self.add(desc, date, tags, uid, item_source);
        }
    }

    pub fn add(
        &mut self,
        desc: Option<String>,
        date: Option<NaiveDate>,
        tags: Option<Vec<String>>,
        uid: Option<i64>,
        item_source: Option<ItemSource>,
    ) -> u32 {
        let next_id = self.get_next_id();
        let desc = desc.expect("Item description cannot be empty!");
        let date = date.unwrap_or(Local::today().naive_local());
        let tags = tags.unwrap_or(vec![]);
        let completed = false;
        let uid = uid.unwrap_or(0);
        let item_source = item_source.unwrap_or(ItemSource::Local);
        let new_item = Item::new(next_id, desc, date, tags, completed, uid, item_source);

        self.items.push(new_item);

        next_id
    }

    pub fn edit(
        &mut self,
        id: u32,
        desc: Option<String>,
        date: Option<NaiveDate>,
        tags: Option<Vec<String>>,
    ) {
        let item_to_edit = self.find_item_by_id(id);
        item_to_edit.edit(desc, date, tags);
    }

    pub fn toggle_completed_batch(&mut self, ids: &Vec<u32>) -> () {
        for id in ids {
            let item = self.find_item_by_id(*id);
            item.toggle_completed();
        }
    }

    pub fn delete_batch(&mut self, ids: Vec<u32>) -> () {
        self.items.retain(|item| !ids.contains(&item.id));
    }

    pub fn delete_batch_by_uids(&mut self, uids: Vec<i64>) -> () {
        self.items.retain(|item| !uids.contains(&item.uid));
    }

    pub fn find_items_uid_by_source(&mut self, source: ItemSource) -> Vec<i64> {
        self.items
            .iter_mut()
            .filter(|_item| _item.item_source == source)
            .map(|_item| _item.uid)
            .collect()
    }

    pub fn find_item_by_id(&mut self, id: u32) -> &mut Item {
        self.items
            .iter_mut()
            .find(|item| item.id == id)
            .expect(&format!("Cannot find item with id: {}", id))
    }

    fn get_next_id(&self) -> u32 {
        self.get_last_id() + 1
    }

    fn get_last_id(&self) -> u32 {
        Self::get_largest_id(&self.items)
    }

    fn get_largest_id(items: &Vec<Item>) -> u32 {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Local;

    #[test]
    fn add_increments_items_length() {
        let mut todo = Todo::new();

        todo.add(
            Some(String::from("Hello, world!")),
            Some(Local::today().naive_local()),
            Some(vec![String::from("tag1")]),
            None,
        );

        assert_eq!(todo.items.len(), 1);
    }

    #[test]
    fn add_increments_last_item_id() {
        let mut todo = Todo::new();
        todo.add(
            Some(String::from("Hello, world!")),
            Some(Local::today().naive_local()),
            Some(vec![String::from("tag1")]),
            None,
        );
        todo.add(
            Some(String::from("Hello, world war II!")),
            Some(Local::today().naive_local()),
            Some(vec![String::from("tag1")]),
            None,
        );

        todo.add(
            Some(String::from("Hello, world war III!")),
            Some(Local::today().naive_local()),
            Some(vec![String::from("tag1")]),
            None,
        );

        assert_eq!(todo.items.last().unwrap().id, 3);
        assert_eq!(todo.items.len(), 3);
    }
}
