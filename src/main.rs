mod net;
mod game;
mod config;
mod util;

use anyhow::Result;
use config::Config;
use game::Game;
use net::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().await?;
    let server = Server::from_config(&config);
    server.start();
    Game::new(server).main_loop().await;
    Ok(())
}