use crate::recipe_item::RecipeItem;

/// Recipe
/// This defines what is required to create a product
pub struct Recipe {
    pub product_id: String,
    pub name: String,
    pub items: Vec<RecipeItem>
}

impl Recipe {
    pub fn new(product_id: String, name: String, items: Vec<RecipeItem>) -> Recipe {
        Recipe {
            product_id,
            name,
            items
        }
    }
}
