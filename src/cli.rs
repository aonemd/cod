use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cod", about = "The command line todo app.")]
pub struct Cli {
    #[structopt(short = "f", long = "file")]
    pub file: Option<String>,

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "list", visible_alias = "l")]
    /// List all items
    List,

    /// Add an item. Date starts with a `@`. Tags start with a `+`
    #[structopt(name = "add", visible_alias = "a")]
    Add { content: Vec<String> },

    /// Edit an item. Attributes not passed will not be changed
    #[structopt(name = "edit", visible_alias = "e")]
    Edit { id: u32, content: Vec<String> },

    /// Toggle completed status of one or more items using their ids
    #[structopt(name = "complete", visible_alias = "c")]
    Complete { ids: Vec<u32> },

    /// Delete one or more items using their ids
    #[structopt(name = "delete", visible_alias = "d")]
    Delete { ids: Vec<u32> },
}
