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
    List,
    #[structopt(name = "add", visible_alias = "a")]
    Add {
        content: Vec<String>,
    },
    #[structopt(name = "edit", visible_alias = "e")]
    Edit {
        id: u32,
        content: Vec<String>,
    },
    #[structopt(name = "complete", visible_alias = "c")]
    Complete {
        ids: Vec<u32>,
    },
}
