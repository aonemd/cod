use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    items: Option<Vec<Item>>,
    projects: Option<Vec<Project>>,
    labels: Option<Vec<Label>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    id: i64,
    project_id: i64,
    content: String,
    description: String,
    priority: usize,
    due: Option<DueDate>,
    labels: Vec<i64>,
    checked: u8,
    is_deleted: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct DueDate {
    date: String,
    string: String,
    is_recurring: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    id: i64,
    name: String,
    is_deleted: u8,
    is_archived: u8,
    is_favorite: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    id: i64,
    name: String,
    is_deleted: u8,
    is_favorite: u8,
}
