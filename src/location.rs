use crate::exit::Exit;
use crate::item::Item;
use crate::npc::NPC;

pub struct Location {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) items: Vec<Item>,
    pub(crate) npcs: Vec<NPC>,
    pub(crate) exits: Vec<Exit>,
}