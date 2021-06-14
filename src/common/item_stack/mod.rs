mod item_type;

use nbt::Value as Nbt;
pub use item_type::ItemType;

#[derive(Clone, Debug)]
pub struct ItemStack {
    pub item: &'static ItemType,
    pub count: u8,
    pub nbt: Option<Nbt>,
}
