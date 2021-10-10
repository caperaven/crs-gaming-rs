pub struct InventoryItem {
    pub product_id: String,
    pub max_quantity: f64
}

impl InventoryItem {
    pub fn new(product_id: String, max_quantity: f64) -> InventoryItem {
        InventoryItem {
            product_id,
            max_quantity
        }
    }
}