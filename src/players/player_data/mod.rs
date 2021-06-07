use anyhow::Result;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};
use uuid::Uuid;

use crate::util::{read_file, write_file};

use crate::inventory::Inventory;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="PascalCase")] 
pub struct PlayerData {
    pub pos: Vector3<f64>,
    #[serde(serialize_with="serialize_rotation")]
    pub rotation: (f32, f32),
    pub inventory: Inventory,
}

impl PlayerData {
    pub async fn load(uuid: Uuid) -> Result<Self> {
        let file = read_file(get_path(uuid)).await?;
        Ok(nbt::from_gzip_reader(file.as_slice())?)
    }

    pub async fn save(&self, uuid: Uuid) -> Result<()> {
        let mut data = vec![];
        nbt::to_gzip_writer(&mut data, self, None)?;
        write_file(get_path(uuid), data.as_slice()).await?;
        Ok(())
    }
}

fn get_path(uuid: Uuid) -> String {
    format!("./world/playerdata/{}.dat", uuid)
}

fn serialize_rotation<S>(rotation: &(f32, f32), serializer: S) 
    -> Result<S::Ok, S::Error> where S: Serializer
{
    let mut seq = serializer.serialize_seq(Some(2))?;
    seq.serialize_element(&rotation.0)?;
    seq.serialize_element(&rotation.1)?;
    seq.end()
}
