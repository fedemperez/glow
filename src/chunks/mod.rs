mod block;
mod chunk;
mod section;
mod chunk_source;
mod world;
mod coords;
mod flat_generator;

pub use chunk::Chunk;
pub use block::Block;
pub use coords::ChunkCoords;
pub use world::World;
use legion::{Resources, systems::Builder};
use flat_generator::FlatGenerator;

pub fn register(schedule: &mut Builder, resources: &mut Resources) {
    resources.insert(World::new(vec![
        Box::new(FlatGenerator),
    ]));
}