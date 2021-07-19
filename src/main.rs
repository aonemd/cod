mod app;
mod cli;
mod config;
mod parser;
mod store;
mod synchronizer;
mod todo;

use app::run;
pub use cli::Cli;
pub use cli::Command;
pub use config::Config;
pub use parser::Parser;
pub use store::yaml_store::YamlStore;
pub use synchronizer::todoist::*;
pub use todo::item::Item;
pub use todo::item_source::ItemSource;
pub use todo::todo::Todo;
pub use todo::todo_serialized::TodoSerialized;

use structopt::StructOpt;

extern crate todoist;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();

    run(cli).await;

    Ok(())
}
