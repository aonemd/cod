use crate::Cli;
use crate::Command;
use crate::Parser;
use crate::Todo;
use crate::TodoSerialized;


use serde_yaml;

pub fn run(cli: Cli) -> () {
    let mut todo = Todo::new();

    match cli.command {
        Command::Add {content} => {
            let parser = Parser::new(&content.join(" "));

            todo.add(parser.desc, parser.date, parser.tags);
            let todo_serialized = TodoSerialized::from(&todo);
            let s = serde_yaml::to_string(&todo_serialized).unwrap();
            println!("Items \"{:?}\" added!", s);

            std::fs::write("todos.yml", s).unwrap();
        }
    }
}
