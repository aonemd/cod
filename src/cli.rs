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
    Add {
        content: Vec<String>,
    },
}
