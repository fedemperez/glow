mod chunks;
mod players;

pub use chunks::{Chunk, ChunkCoords};

use std::time::{Duration, Instant};

use legion::*;
use tokio::time::sleep;

use crate::{game::chunks::FlatGenerator, net::Server};
use chunks::ChunkWorld;

const TICK_INTERVAL: Duration = Duration::from_millis(1000 / 20);

pub struct Game {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl Game {
    pub fn new(server: Server) -> Self {
        let mut resources = Resources::default();
        resources.insert(server);
        resources.insert(ChunkWorld::new(vec![
            Box::new(FlatGenerator),
        ]));
        let mut schedule = Schedule::builder();
        players::register_systems(&mut schedule);
        let schedule = schedule.build();
        Self {
            world: World::default(),
            resources,
            schedule,
        }
    }

    pub async fn main_loop(mut self) {
        loop {
            let start = Instant::now();
            self.schedule.execute(&mut self.world, &mut self.resources);
            if start.elapsed() < TICK_INTERVAL {
                sleep(TICK_INTERVAL - start.elapsed()).await;
            }
        }
    }
}