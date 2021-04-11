use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cod", about = "The command line todo app.")]
pub struct Cli {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {}
