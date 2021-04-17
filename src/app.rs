use crate::Cli;
use crate::Command;
use crate::Parser;
use crate::Todo;

pub fn run(cli: Cli) -> () {
    let mut todo = Todo::new();

    match cli.command {
        Command::Add {content} => {
            let parser = Parser::new(&content.join(" "));

            todo.add(parser.desc, parser.date, parser.tags);
            println!("Items \"{:?}\" added!", todo.items);
        }
    }
}
