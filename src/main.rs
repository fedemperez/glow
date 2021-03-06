mod net;
mod players;
mod chunks;
mod config;
mod util;
mod entities;
mod buckets;
mod inventory;
mod items;
mod serialization;
mod tags;
mod blocks;

use std::{
    sync::atomic::{AtomicBool, Ordering}, 
    time::{Duration, Instant}};

use anyhow::Result;
use config::Config;
use legion::*;

use net::Server;
use tokio::time::sleep;

const PRINT_TIMING: bool = false;
const TICK_INTERVAL: Duration = Duration::from_millis(1000 / 20);
static STOP_SIGNAL: AtomicBool = AtomicBool::new(false);

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().await?;
    let server = Server::start(&config);
    let mut resources = Resources::default();
    resources.insert(server);
    let mut schedule = Schedule::builder();
    players::register_early(&mut schedule, &mut resources);
    chunks::register(&mut schedule, &mut resources);
    entities::register(&mut schedule, &mut resources);
    buckets::register(&mut schedule, &mut resources);
    players::register_late(&mut schedule, &mut resources);
    let mut schedule = schedule.build();
    let mut world = World::default();
    ctrlc::set_handler(|| {
        STOP_SIGNAL.store(true, Ordering::Relaxed)
    }).expect("Error setting ctrl + c handler");
    let mut last_msg = Instant::now();
    loop {
        let start = Instant::now();
        schedule.execute(&mut world, &mut resources);
        if PRINT_TIMING && last_msg.elapsed() > Duration::from_secs(2) {
            println!("Last update took {} micros", 
                start.elapsed().as_micros());
            last_msg = Instant::now();
        }
        if start.elapsed() < TICK_INTERVAL {
            sleep(TICK_INTERVAL - start.elapsed()).await;
        }
        if STOP_SIGNAL.load(Ordering::Relaxed) {
            println!("\nStopping...");
            players::on_stop(&mut world, &mut resources).await;
            chunks::on_stop(&mut resources).await;
            break Ok(());
        }
    }
}
