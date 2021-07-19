use crate::ItemSource;
use crate::Todo;

use std::collections::HashMap;

use chrono::NaiveDate;
use serde_json::json;
use uuid::Uuid;

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

pub enum SyncUpOp {
    ItemAdd,
}

pub async fn sync_up(todo: &mut Todo, ids: Vec<u32>, op: SyncUpOp, token: String) -> () {
    let client = todoist::SyncApi::new(token);

    let payload = client.read_resources(Some(vec!["projects"])).await.unwrap();
    let mut projects: HashMap<String, i64> = HashMap::new();
    for (_, _project) in payload.projects.unwrap().into_iter().enumerate() {
        projects.insert(_project.name, _project.id);
    }

    match op {
        ItemAdd => {
            let commands = ids
                .into_iter()
                .map(|id| todoist::types::WriteCommand {
                    r#type: "item_add".to_string(),
                    args: convert_item_to_hash(&todo.find_item_by_id(id), &projects),
                    uuid: Uuid::new_v4().to_string(),
                    temp_id: Uuid::new_v4().to_string(),
                })
                .collect::<Vec<todoist::types::WriteCommand>>();
            let commands = todoist::types::WriteCommands(commands);
            client.write_resources(commands).await;
        }
    }
}

fn convert_item_to_hash(item: &crate::Item, projects: &HashMap<String, i64>) -> serde_json::Value {
    let cloned = item.clone();

    json!({
        "content": cloned.desc,
        "project_id": projects.get(&cloned.tags[0]),
        "due": {
            "date": cloned.date,
        }
    })
}
