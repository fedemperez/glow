mod item_id;

use nbt::Value as Nbt;
pub use item_id::ItemId;

#[derive(Clone, Debug)]
pub struct ItemStack {
    pub id: ItemId,
    pub count: u8,
    pub nbt: Option<Nbt>,
}
