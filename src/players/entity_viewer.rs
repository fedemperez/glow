use std::collections::{HashSet, HashMap};
use uuid::Uuid;
use legion::*;
use world::SubWorld;
use crate::buckets::{EntityTracker, Observer};
use crate::entities::{EntityId, Position, Rotation};
use crate::net::PlayerConnection;
use crate::events::ServerEvent;

const VIEW_RANGE: u32 = 6 * 16;

#[system]
#[read_component(Position)]
#[read_component(Rotation)]
#[read_component(Uuid)]
#[read_component(EntityId)]
#[write_component(PlayerConnection)]
pub fn send_visible_entities(world: &mut SubWorld, 
    #[resource] tracker: &EntityTracker) 
{
    let query = <(&Position, &PlayerConnection, &Observer)>::query();
    for (pos, conn, observer) in query.iter_mut(world) {
        let events = observer.update(&pos.0, tracker);
        for event in events {
            conn.send(ev)
        }
    }
}
