mod cli;
mod app;
mod parser;
mod store;
mod todo;

pub use cli::Cli;
pub use cli::Command;
pub use parser::Parser;
pub use todo::todo::Todo;
pub use todo::todo_serialized::TodoSerialized;
pub use store::yaml_store::YamlStore;
use app::run;

use structopt::StructOpt;

extern crate todoist;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();

    run(cli).await;

    Ok(())
}
