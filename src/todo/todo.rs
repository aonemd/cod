use super::item::Item;

pub struct Todo {
    items: Vec<Item>,
}

impl Todo {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, desc: String) {
        let next_id = self.get_next_id();
        let new_item = Item::new(next_id, desc);
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

    #[test]
    fn add_increments_items_length() {
        let mut todo = Todo::new();

        todo.add(String::from("Hello, world!"));

        assert_eq!(todo.items.len(), 1);
    }

    #[test]
    fn add_increments_last_item_id() {
        let mut todo = Todo::new();
        todo.add(String::from("Hello, world!"));
        todo.add(String::from("Hello, world war II!"));

        todo.add(String::from("Hello, world war III!"));

        assert_eq!(todo.items.last().unwrap().id, 3);
    }
}
