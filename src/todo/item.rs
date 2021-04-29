use chrono::{NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
}
