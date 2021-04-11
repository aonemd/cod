mod cli;

use cli::Cli;

use structopt::StructOpt;

fn main() {
    let cli = Cli::from_args();
}
