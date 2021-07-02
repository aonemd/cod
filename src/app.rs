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
            }
            todo.list();
        }
        Command::Add { content } => {
            let parser = Parser::new(&content.join(" "));

            todo.add(parser.desc, parser.date, parser.tags, Some(0));
            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        }
        Command::Edit { id, content } => {
            let parser = Parser::new(&content.join(" "));

            todo.edit(id, parser.desc, parser.date, parser.tags);
            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        }
        Command::Complete { ids } => {
            todo.toggle_completed_batch(ids);
            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        }
        Command::Delete { ids } => {
            todo.delete_batch(ids);
            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        }
    }
}
