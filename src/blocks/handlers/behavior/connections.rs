use std::collections::{BTreeMap, HashMap};
use crate::{chunks::WorldView, util::cardinal_to_vec};

pub fn update_connections(props: &mut BTreeMap<String, String>, 
    view: &WorldView, true_alias: &str, false_alias: &str)
{
    let conns = get_connections(&view);
    for (cardinal, value) in conns {
        props.insert(cardinal, if value {
            true_alias
        } else {
            false_alias
        }.into());
    }
}

fn get_connections(view: &WorldView) -> HashMap<String, bool> {
    let mut map = HashMap::new();
    for cardinal in &["north", "south", "east", "west"] {
        let dir = cardinal_to_vec(cardinal);
        let connected = view.get(dir.x, dir.y, dir.z).material.solid;
        map.insert(cardinal.to_string(), connected);
    }
    map
}
