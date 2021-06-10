mod chunk;
mod section;
mod chunk_source;
mod world;
mod coords;
mod flat_generator;
mod palette;
mod loading;
pub mod events;
mod chunk_data;

use std::io::Write;

pub use chunk_data::ChunkData;
pub use coords::ChunkCoords;
pub use world::World;
use legion::{systems::Builder, Resources};
use flat_generator::FlatGenerator;

use crate::chunks::loading::AnvilChunkLoader;

pub fn register(schedule: &mut Builder, resources: &mut Resources) {
    world::register(schedule);
    resources.insert(World::new(vec![
        Box::new(AnvilChunkLoader::new()),
        Box::new(FlatGenerator),
    ]));
}

pub async fn on_stop(resources: &mut Resources) {
    print!("Saving chunks...         ");
    let _ = std::io::stdout().flush();
    resources.get::<World>().unwrap().save_all();
    println!("Done");
}