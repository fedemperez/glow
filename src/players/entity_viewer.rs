use std::collections::HashSet;

use legion::*;
use crate::entities::{Position, SpatialHashMap};
use crate::net::PlayerConnection;

const VIEW_RANGE: u32 = 6 * 16;

#[system(for_each)]
pub fn send_visible_entities(pos: &Position, view: &mut EntityViewer, 
               conn: &mut PlayerConnection, #[resource] map: &mut SpatialHashMap) 
{
    let visible = map.get_close_entities(&pos.0, VIEW_RANGE);
    println!("{} entities close", visible.len());
}

pub struct EntityViewer {
    last_seen: HashSet<Entity>,
}

impl EntityViewer {
    pub fn new() -> Self {
        Self {
            last_seen: HashSet::new(),
        }
    }
}