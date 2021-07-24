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
            let mut commands: Vec<todoist::types::WriteCommand> = vec![];
            let mut temp_id_to_id_map: HashMap<String, u32> = HashMap::new();
            for id in ids {
                let temp_id = Uuid::new_v4().to_string();
                let command = todoist::types::WriteCommand {
                    r#type: "item_add".to_string(),
                    args: convert_item_to_hash(&mut todo.find_item_by_id(*id), &projects),
                    uuid: Uuid::new_v4().to_string(),
                    temp_id: Some(temp_id.clone()),
                };

                commands.push(command);
                temp_id_to_id_map.insert(temp_id, *id);
            }
            let commands = todoist::types::WriteCommands(commands);
            let write_response = client.write_resources(commands).await.unwrap();
            for (k, v) in write_response.temp_id_mapping.into_iter() {
                let id = match temp_id_to_id_map.get(&k) {
                    Some(id) => id,
                    None => continue,
                };
                let uid = v;
                let item = todo.find_item_by_id(*id);
                item.edit_item_uid(uid);
                item.edit_item_source(ItemSource::Todoist);
            }
        }
    }
}

fn convert_item_to_hash(
    item: &mut crate::Item,
    projects: &HashMap<String, i64>,
) -> serde_json::Value {
    let cloned = item.clone();

    json!({
        "content": cloned.desc,
        "project_id": projects.get(&cloned.tags[0]),
        "due": {
            "date": cloned.date,
        }
    })
}
