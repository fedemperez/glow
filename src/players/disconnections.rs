use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender, channel};

use uuid::Uuid;
use legion::*;
use systems::CommandBuffer;
use world::SubWorld;
use crate::buckets::EntityTracker;
use crate::entities::{EntityId, Position, Rotation};
use crate::inventory::Inventory;
use crate::players::player_data::PlayerData;

use super::player_list::PlayerList;
use crate::entities::Name;

pub struct DisconnectionQueue {
    sender: Mutex<Sender<(Entity, String)>>,
    receiver: Mutex<Receiver<(Entity, String)>>,
}

impl DisconnectionQueue {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver),
        }
    }

    pub fn send(&self, entity: Entity, reason: String) {
        self.sender.lock().unwrap().send((entity, reason));
    } 
}

#[system]
#[read_component(Name)]
#[read_component(Uuid)]
#[read_component(Position)]
#[read_component(Rotation)]
#[read_component(Inventory)]
pub fn handle_disconnections(world: &mut SubWorld, #[resource] tracker: &EntityTracker, 
    #[resource] queue: &DisconnectionQueue, cmd: &mut CommandBuffer) 
{
    for (entity, reason) in queue.receiver.lock().unwrap().try_recv() {
        let entry = world.entry_ref(entity).unwrap();
        let name = entry.get_component::<Name>().unwrap().0.clone();
        println!("{} disconnected, reason: {}", name, reason);
        let uuid = *entry.get_component::<Uuid>().unwrap();
        let position = entry.get_component::<Position>().unwrap().0;
        let rotation = entry.get_component::<Rotation>().unwrap();
        let inventory = entry.get_component::<Inventory>().unwrap().clone();
        let data = PlayerData {
            pos: position,
            rotation: (rotation.0, rotation.1),
            inventory,
        };
        tokio::spawn(async move {
            data.save(uuid).await
        });
        cmd.exec_mut(move |world, resources| {
            remove_player(entity, world, resources);
        });
    }
}

fn remove_player(entity: Entity, world: &mut World, resources: &mut Resources) {
    if let Some(entry) = world.entry(entity) {
        (|| {
            let mut list = resources.get_mut::<PlayerList>()?;
            let uuid = entry.get_component::<Uuid>().ok()?;
            list.remove(*uuid);
            Some(())
        })();
        (|| {
            let mut tracker = resources.get_mut::<EntityTracker>()?;
            let id = entry.get_component::<EntityId>().ok()?;
            let pos = entry.get_component::<Position>().ok()?;
            tracker.remove(id.0, &pos.0);
            Some(())
        })();
    }
    world.remove(entity);
}