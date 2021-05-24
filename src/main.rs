mod net;
mod game;
mod config;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::net::Server;
use anyhow::{Result, Error};
use config::Config;
use legion::*;
use std::time::{Instant, Duration};
use tokio::time::sleep;

const TICK_INTERVAL: Duration = Duration::from_millis(1000 / 20);

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().await?;
    tokio::spawn(Server::from_config(&config).serve());
    game_loop().await;
    Ok(())
}

async fn game_loop() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder().build();
    loop {
        let start = Instant::now();
        schedule.execute(&mut world, &mut resources);
        if start.elapsed() < TICK_INTERVAL {
            sleep(TICK_INTERVAL - start.elapsed()).await;
        }
    }
}