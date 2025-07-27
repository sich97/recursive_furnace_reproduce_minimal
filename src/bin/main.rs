use loco_rs::cli;
use migration::Migrator;
use recursive_furnace_reproduce_minimal::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App, Migrator>().await
}
