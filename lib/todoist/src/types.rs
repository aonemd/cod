use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub items: Option<Vec<Item>>,
    pub projects: Option<Vec<Project>>,
    pub labels: Option<Vec<Label>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub project_id: i64,
    pub content: String,
    pub description: String,
    pub priority: usize,
    pub due: Option<DueDate>,
    pub labels: Vec<i64>,
    pub checked: u8,
    pub is_deleted: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDate {
    pub date: String,
    pub string: String,
    pub is_recurring: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub is_deleted: u8,
    pub is_archived: u8,
    pub is_favorite: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub name: String,
    pub is_deleted: u8,
    pub is_favorite: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteCommands(pub Vec<WriteCommand>);

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteCommand {
    pub r#type: String,
    pub args: serde_json::Value,
    pub uuid: String,
    pub temp_id: String,
}
