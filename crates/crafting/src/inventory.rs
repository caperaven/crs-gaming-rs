use crate::inventory_item::InventoryItem;

pub struct Inventory {
    pub id: String,
    pub items: Vec<InventoryItem>
}

impl Inventory {
    pub fn new(id: String, items: Vec<InventoryItem>) -> Inventory {
        Inventory {
            id,
            items
        }
    }
}