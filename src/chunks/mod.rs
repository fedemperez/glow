mod chunk;
mod world;
mod coords;
mod loading;
mod data;
mod saving;
mod view;
pub mod events;

use std::io::Write;

pub use data::ChunkData;
pub use coords::ChunkCoords;
pub use world::World;
pub use view::WorldView;
use legion::{systems::Builder, Resources};
use loading::{FlatGenerator, AnvilChunkLoader};

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
    resources.get_mut::<World>().unwrap().save_all();
    println!("Done");
}
