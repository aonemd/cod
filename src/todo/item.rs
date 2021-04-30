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
mod tests {
    use super::*;
}
