use chrono::{NaiveDate};

#[derive(Debug)]
pub struct Item {
    pub id: u32,
    pub desc: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
}

impl Item {
    pub fn new(id: u32, desc: String, date: NaiveDate, tags: Vec<String>) -> Self {
        Self {
            id,
            desc,
            date,
            tags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
