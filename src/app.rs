use crate::Cli;
use crate::Command;
use crate::Parser;
use crate::Todo;
use crate::TodoSerialized;
use crate::YamlStore;

pub fn run(cli: Cli) -> () {
    let store = &YamlStore::new(None);
    let todo_serialized: TodoSerialized = *store.read();

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
        }
    }
}
