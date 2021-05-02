use crate::Cli;
use crate::Command;
use crate::Parser;
use crate::Todo;
use crate::TodoSerialized;
use crate::YamlStore;

pub fn run(cli: Cli) -> () {
    let store = &YamlStore::new(cli.file);
    let todo_serialized: TodoSerialized = match store.read() {
        Ok(t) => *t,
        Err(_) => TodoSerialized::default(),
    };

    let mut todo = Todo::from(todo_serialized);

    match cli.command {
        Command::List => {
            todo.list();
        },
        Command::Add {content} => {
            let parser = Parser::new(&content.join(" "));

            todo.add(parser.desc, parser.date, parser.tags);
            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        },
        Command::Edit {id, content} => {
            let parser = Parser::new(&content.join(" "));

            todo.edit(id, parser.desc, parser.date, parser.tags);
            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        },
        Command::Complete {ids} => {
            todo.toggle_completed_batch(ids);
            let todo_serialized = TodoSerialized::from(&todo);
            store.write(&todo_serialized);
        },
    }
}
