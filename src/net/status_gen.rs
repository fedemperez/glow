use serde::Serialize;
use serde_json::json;

pub fn gen_status_str(motd: &String, player_list: &Vec<String>) -> String {
    json!({
        "version": {
            "name": "1.16.5",
            "protocol": 754
        },
        "players": {
            "max": 100,
            "online": player_list.len(),
            "sample": player_list.iter().map(|player| PlayerSample::new(player))
                .collect::<Vec<PlayerSample<'_>>>()
        },
        "description": {
            "text": motd
        },
        "favicon": "data:image/png;base64,<data>"
    }).to_string()
}

#[derive(Serialize)]
struct PlayerSample<'a> {
    name: &'a str,
    id: &'a str,
}

impl<'a> PlayerSample<'a> {
    fn new(name: &'a String) -> Self {
        Self {
            name: name.as_str(),
            id: "4566e69f-c907-48ee-8d71-d7ba5aa00d20",
        }
    }
}