use crate::synchronizer;
use crate::Cli;
use crate::Command;
use crate::Config;
use crate::Parser;
use crate::Todo;
use crate::TodoSerialized;
use crate::YamlStore;

pub async fn run(cli: Cli) -> () {
    let store = &YamlStore::new(cli.file);
    let todo_serialized: TodoSerialized = match store.read() {
        Ok(t) => *t,
        Err(_) => TodoSerialized::default(),
    };

    let config = Config::new();

    let mut todo = Todo::from(todo_serialized);

    match cli.command {
        Command::List => {
            if let Some(token) = config.todoist_token {
                synchronizer::todoist::sync_down(&mut todo, token).await;

                let todo_serialized = TodoSerialized::from(&todo);
                store.write(&todo_serialized);
            }

            todo.list();
        }
        Command::Add { content } => {
            let parser = Parser::new(&content.join(" "));

            let item_id = todo.add(parser.desc, parser.date, parser.tags, Some(0), None);

            if let Some(token) = config.todoist_token {
                synchronizer::todoist::sync_up(
                    &mut todo,
                    &vec![item_id],
                    synchronizer::todoist::SyncUpOp::ItemAdd,
                    token,
                )
                .await;
            }

            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        }
        Command::Edit { id, content } => {
            let parser = Parser::new(&content.join(" "));

            todo.edit(id, parser.desc, parser.date, parser.tags);

            if let Some(token) = config.todoist_token {
                synchronizer::todoist::sync_up(
                    &mut todo,
                    &vec![id],
                    synchronizer::todoist::SyncUpOp::ItemUpdate,
                    token,
                )
                .await;
            }

            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        }
        Command::Complete { ids } => {
            todo.toggle_completed_batch(&ids);

            if let Some(token) = config.todoist_token {
                synchronizer::todoist::sync_up(
                    &mut todo,
                    &ids,
                    synchronizer::todoist::SyncUpOp::ItemUpdate,
                    token,
                )
                .await;
            }

            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        }
        Command::Delete { ids } => {
            if let Some(token) = config.todoist_token {
                synchronizer::todoist::sync_up(
                    &mut todo,
                    &ids,
                    synchronizer::todoist::SyncUpOp::ItemDelete,
                    token,
                )
                .await;
            }

            todo.delete_batch(ids);
            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        }
    }
}
