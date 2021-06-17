mod ids;

use ids::*;
use std::{collections::HashMap, fs::{DirEntry, File, read_dir}, io::Write, path::Path};
use serde::Deserialize;

fn main() {
    let mut output = File::create("tag_packet.dat").unwrap();
    let tag_types = vec![
        ("tags/blocks", &*BLOCK_IDS),
        ("tags/items", &*ITEM_IDS),
        ("tags/fluids", &*FLUID_IDS),
        ("tags/entity_types", &*ENTITY_IDS),
    ];
    for (dir, id_map) in tag_types {
        let tags = get_tags_from_dir(dir, id_map);
        write_tags(tags, &mut output);
    }
}

fn get_tags_from_dir(dir: &str, id_map: &HashMap<String, u32>) -> Vec<Tag> {
    read_dir(dir)
        .expect(format!("Cannot read directory {}", dir).as_str())
        .map(|path| file_to_tag(path.unwrap(), &id_map))
        .collect()
}

fn file_to_tag(path: DirEntry, id_map: &HashMap<String, u32>) -> Tag {
    Tag {
        name: path.file_name().into_string().unwrap()
            .replace(".json", ""), 
        entries: read_entries(path.path(), id_map),
    }
}

fn read_entries(path: impl AsRef<Path>, id_map: &HashMap<String, u32>) 
    -> Vec<u32> 
{
    let file = File::open(path.as_ref()).unwrap();
    let json: TagJson = serde_json::from_reader(file).unwrap();
    let mut result = vec![];
    for value in json.values {
        if let Some(value) = id_map.get(&value).cloned() {
            result.push(value);
        } else {
            let name = value.replace("#minecraft:", "");
            let dir = path.as_ref().parent().unwrap();
            let path = dir.join(format!("{}.json", name));
            let other_tag = read_entries(path, id_map);
            result.extend(other_tag);
        }
    }
    result
}

fn write_tags<W>(tags: Vec<Tag>, dest: &mut W) where W: Write {
    write_varint(tags.len() as u32, dest);
    for tag in tags {
        tag.write(dest);
    }
}

#[derive(Deserialize)]
struct TagJson {
    replace: bool,
    values: Vec<String>,
}

struct Tag {
    name: String,
    entries: Vec<u32>,
}

impl Tag {
    fn write<W>(&self, dest: &mut W) where W: Write {
        let name = self.name.as_bytes();
        write_varint(name.len() as u32, dest);
        dest.write(name);
        write_varint(self.entries.len() as u32, dest);
        for entry in &self.entries {
            write_varint(*entry, dest);
        }
    }
}

fn write_varint<W>(mut value: u32, dest: &mut W) where W: Write {
    loop {
        let mut byte = value as u8 & 0b01111111;
        value >>= 7;
        if value != 0 {
            byte |= 0b10000000;
        }
        dest.write(&[byte]);
        if value == 0 {
            break
        }
    }
}
