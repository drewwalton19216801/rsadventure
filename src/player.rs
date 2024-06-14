use crate::item::Item;
pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) inventory: Vec<Item>,
    pub(crate) current_location: usize,
}