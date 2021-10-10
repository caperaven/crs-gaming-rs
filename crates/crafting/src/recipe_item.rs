/// RecipeItem
/// This is used in a recipe to define the raw material required and what quantity is needed
pub struct RecipeItem {
    pub material_id: String,
    pub quantity: f64
}

impl RecipeItem {
    pub fn new(material_id: String, quantity: f64) -> RecipeItem {
        RecipeItem {
            material_id,
            quantity
        }
    }
}