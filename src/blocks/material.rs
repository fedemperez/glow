use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BlockMaterial {
    #[serde(skip)]
    pub name: String,
    pub push_reaction: PushReaction,
    pub blocks_motion: bool,
    pub flammable: bool,
    pub liquid: bool,
    pub solid_blocking: bool,
    pub replaceable: bool,
    pub solid: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="lowercase")]
pub enum PushReaction {
    Normal,
    Block,
    Destroy,
}
