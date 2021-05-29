use legion::*;
use world::SubWorld;
use crate::net::{PlayerConnection, Server, ServerEvent};
use std::mem::take;
use std::collections::HashMap;
use uuid::Uuid;

#[system]
#[read_component(PlayerConnection)]
fn update_player_list(world: &SubWorld, #[resource] list: &mut PlayerList, 
                      #[resource] server: &mut Server)
{
    let updates = list.flush_updates();
    if updates.len() > 0 {
        server.update_list(list.count(), list.get_sample());
        for update in updates {
            let mut query = <(&PlayerConnection,)>::query();
            query.for_each(world, |(conn,)| {
                match &update {
                    PlayerListUpdate::Add(uuid, name) => {
                        conn.send(ServerEvent::AddPlayer(*uuid, name.clone()));
                    }
                    PlayerListUpdate::Remove(uuid) => {
                        conn.send(ServerEvent::RemovePlayer(*uuid));
                    }
                }
            });
        }
    }
}

pub struct PlayerList {
    players: HashMap<Uuid, String>,
    pending_updates: Vec<PlayerListUpdate>,
}

impl PlayerList {
    pub fn new() -> Self {
        Self{
            players: HashMap::new(),
            pending_updates: vec![],
        }
    }

    pub fn add(&mut self, uuid: Uuid, name: String) {
        self.pending_updates.push(PlayerListUpdate::Add(uuid, name));
    }

    pub fn remove(&mut self, uuid: Uuid) {
        self.pending_updates.push(PlayerListUpdate::Remove(uuid));
    }

    pub fn flush_updates(&mut self) -> Vec<PlayerListUpdate> {
        let updates = take(&mut self.pending_updates);
        for update in &updates {
            update.apply(self);
        }
        updates
    }

    pub fn get_sample(&self) -> Vec<String> {
        (&self.players).into_iter()
            .take(5)
            .map(|s| s.1.clone())
            .collect()
    }

    pub fn count(&self) -> usize {
        self.players.len()
    }

    pub fn get_players(&self) -> &HashMap<Uuid, String> {
        &self.players
    }
}

#[derive(Debug, Clone)]
pub enum PlayerListUpdate {
    Add(Uuid, String),
    Remove(Uuid),
}

impl PlayerListUpdate {
    fn apply(&self, list: &mut PlayerList) {
        match self {
            PlayerListUpdate::Add(uuid, name) => {
                list.players.insert(*uuid, name.clone());
            }
            PlayerListUpdate::Remove(uuid) => {
                list.players.remove(uuid);
            }
        }
    }
}