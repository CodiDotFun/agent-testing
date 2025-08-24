use clap::Parser;

use command::Command;

mod command;
pub mod utils;

#[derive(Parser)]
struct App {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let app = App::parse();
