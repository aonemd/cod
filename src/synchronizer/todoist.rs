// use std::collections::HashMap;
// let client = todoist::SyncApi::new("73d28ef601726f56fba0b52e5ca7d61f0caa6e0b".to_string());
// println!("{:#?}", client.read_resources(None).await);
//
// let mut item: HashMap<String, String> = HashMap::new();
// item.insert("content".to_string(), "test rust!!!".to_string());
// let command = todoist::types::WriteCommand {
//     r#type: "item_add".to_string(),
//     args: item,
//     uuid: "381e601f-0ef3-4ed6-bf95-58f896d1a314".to_string(),
//     temp_id: "381e601f-0ef3-4ed6-bf95-58f896d1a314".to_string(),
// };
// let commands = todoist::types::WriteCommands(vec![command]);
// println!("{:#?}", client.write_resources(commands).await);

use crate::ItemSource;
use crate::Todo;

use std::collections::HashMap;

use chrono::NaiveDate;

pub async fn sync_down(todo: &mut Todo, token: String) -> () {
    let client = todoist::SyncApi::new(token);

    let payload = client.read_resources(None).await.unwrap();

    let mut projects: HashMap<i64, String> = HashMap::new();
    for (_, _project) in payload.projects.unwrap().into_iter().enumerate() {
        projects.insert(_project.id, _project.name);
    }

    let original_todoist_item_uids = todo.find_items_uid_by_source(ItemSource::Todoist);

    let items: Vec<todoist::Item> = payload.items.unwrap();
    let mut synced_todoist_item_uids = vec![];
    for (_, _item) in items.into_iter().enumerate() {
        let desc = Some(_item.content);
        let date = match _item.due.as_ref() {
            Some(due_date) => Some(NaiveDate::parse_from_str(&due_date.date, "%Y-%m-%d").unwrap()),
            _ => None,
        };
        let project = projects.get(&_item.project_id).unwrap();
        let tags: Option<Vec<String>> = Some(vec![project.to_string()]);
        let uid = Some(_item.id);

        todo.edit_or_add(desc, date, tags, uid, Some(ItemSource::Todoist));

        synced_todoist_item_uids.push(_item.id);
    }

    // these are the items deleted from Todoist and were not sent with the API response
    let item_uids_to_delete: Vec<i64> = original_todoist_item_uids
        .iter()
        .filter(|&x| !synced_todoist_item_uids.contains(x))
        .cloned()
        .collect();

    todo.delete_batch_by_uids(item_uids_to_delete);
}