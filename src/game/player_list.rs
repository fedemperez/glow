use std::{collections::HashSet, mem::take};

pub struct PlayerList {
    players: HashSet<String>,
    pending_updates: Vec<PlayerListUpdate>,
}

impl PlayerList {
    pub fn new() -> Self {
        Self{
            players: HashSet::new(),
            pending_updates: vec![],
        }
    }

    pub fn add(&mut self, player: String) {
        self.pending_updates.push(PlayerListUpdate::Add(player));
    }

    pub fn remove(&mut self, player: String) {
        self.pending_updates.push(PlayerListUpdate::Remove(player));
    }

    pub fn flush_updates(&mut self) -> Vec<PlayerListUpdate> {
        let updates = take(&mut self.pending_updates);
        for update in &updates {
            update.apply(self);
        }
        updates
    }
}

#[derive(Debug, Clone)]
pub enum PlayerListUpdate {
    Add(String),
    Remove(String),
}

impl PlayerListUpdate {
    fn apply(&self, list: &mut PlayerList) {
        match self {
            PlayerListUpdate::Add(name) => {
                list.players.insert(name.clone());
            }
            PlayerListUpdate::Remove(name) => {
                list.players.remove(&name.clone());
            }
        }
    }
}