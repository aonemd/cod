use std::cmp::Ordering;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub desc: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub completed: bool,
}

impl Item {
    pub fn new(id: u32, desc: String, date: NaiveDate, tags: Vec<String>, completed: bool) -> Self {
        Self {
            id,
            desc,
            date,
            tags,
            completed,
        }
    }

    pub fn edit(&mut self, desc: Option<String>, date: Option<NaiveDate>, tags: Option<Vec<String>>) -> () {
        match desc {
            Some(d) => self.desc = d,
            None => {}
        };

        match date {
            Some(d) => self.date = d,
            None => {}
        };

        match tags {
            Some(mut t) => self.tags.append(&mut t),
            None => {}
        };
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date).then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.id == other.id
    }
}

#[cfg(test)]
mod item_tests {
    use super::*;

    use chrono::{Local};

    #[test]
    fn test_edit() -> () {
        let today = Local::today().naive_local();
        let mut item = Item::new(1, "Hello".to_string(), today, vec!["work".to_string()], false);

        let expected_desc = String::from("Hello, world!");
        let expected_date = today.pred();
        let expected_tags = vec!["work".to_string(), "personal".to_string()];

        item.edit(Some(expected_desc.clone()), Some(expected_date.clone()), Some(vec!["personal".to_string()]));

        assert_eq!(item.desc, expected_desc);
        assert_eq!(item.date, expected_date);
        assert_eq!(item.tags, expected_tags);
    }

    #[test]
    fn test_edit_desc_only() -> () {
        let today = Local::today().naive_local();
        let mut item = Item::new(1, "Hello".to_string(), today, vec!["work".to_string()], false);

        let expected_desc = String::from("Hello, world!");
        let expected_date = today;
        let expected_tags = vec!["work".to_string()];

        item.edit(Some(expected_desc.clone()), None, None);

        assert_eq!(item.desc, expected_desc);
        assert_eq!(item.date, expected_date);
        assert_eq!(item.tags, expected_tags);
    }

    #[test]
    fn test_edit_date_only() -> () {
        let today = Local::today().naive_local();
        let mut item = Item::new(1, "Hello".to_string(), today, vec!["work".to_string()], false);

        let expected_desc = String::from("Hello");
        let expected_date = today.succ();
        let expected_tags = vec!["work".to_string()];

        item.edit(None, Some(expected_date.clone()), None);

        assert_eq!(item.desc, expected_desc);
        assert_eq!(item.date, expected_date);
        assert_eq!(item.tags, expected_tags);
    }


    #[test]
    fn test_edit_tags_only_appends_to_existing_tags() -> () {
        let today = Local::today().naive_local();
        let mut item = Item::new(1, "Hello".to_string(), today, vec!["work".to_string()], false);

        let expected_desc = String::from("Hello");
        let expected_date = today;
        let expected_tags = vec!["work".to_string(), "personal".to_string()];

        item.edit(None, None, Some(vec!["personal".to_string()]));

        assert_eq!(item.desc, expected_desc);
        assert_eq!(item.date, expected_date);
        assert_eq!(item.tags, expected_tags);
    }
}
