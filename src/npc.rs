use crate::item::Item;

pub struct NPC {
    pub(crate) name: String,
    dialogue: String,
    items: Vec<Item>,
    location: usize
}