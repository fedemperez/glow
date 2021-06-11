use serde::Serialize;
use serde_json::json;

pub struct ServerInfo {
    motd: String,
    player_sample: Vec<PlayerSample>,
    player_count: usize,
}

impl ServerInfo {
    pub fn new(motd: &str) -> Self {
        Self {
            motd: motd.to_owned(),
            player_sample: vec![],
            player_count: 0,
        }
    }

    pub fn update_players(&mut self, sample: Vec<String>, count: usize) {
        self.player_sample = sample.into_iter().map(|name| {
            PlayerSample::new(name)
        }).collect();
        self.player_count = count;
    }

    pub fn to_status_str(&self) -> String {
        json!({
            "version": {
                "name": "1.16.5",
                "protocol": 754
            },
            "players": {
                "max": 100,
                "online": self.player_count,
                "sample": self.player_sample,
            },
            "description": {
                "text": self.motd,
            },
            "favicon": "data:image/png;base64,<data>"
        }).to_string()
    }
}

#[derive(Serialize)]
struct PlayerSample {
    name: String,
    id: &'static str,
}

impl PlayerSample {
    fn new(name: String) -> Self {
        Self {
            name,
            id: "4566e69f-c907-48ee-8d71-d7ba5aa00d20",
        }
    }
}
