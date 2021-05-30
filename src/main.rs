mod net;
mod players;
mod chunks;
mod config;
mod util;
mod entities;
mod events;

use std::time::{Duration, Instant};

use anyhow::Result;
use config::Config;
use legion::*;

use net::Server;
use tokio::time::sleep;

const TICK_INTERVAL: Duration = Duration::from_millis(1000 / 20);

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().await?;
    let server = Server::start(&config);
    let mut resources = Resources::default();
    resources.insert(server);
    let mut schedule = Schedule::builder();
    chunks::register(&mut schedule, &mut resources);
    players::register(&mut schedule, &mut resources);
    entities::register(&mut schedule, &mut resources);
    let mut schedule = schedule.build();
    let mut world = World::default();
    loop {
        let start = Instant::now();
        schedule.execute(&mut world, &mut resources);
        if start.elapsed() < TICK_INTERVAL {
            sleep(TICK_INTERVAL - start.elapsed()).await;
        }
    }
}