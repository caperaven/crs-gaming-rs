use crate::named_item::NamedItem;
use crate::recipe::Recipe;
use std::collections::HashMap;
use crate::material::Material;

mod recipe;
mod named_item;
mod recipe_item;
mod inventory_item;
mod inventory;
mod material;

type Materials = HashMap<String, Material>;

pub fn craft(recipe: &Recipe, materials: &Vec<NamedItem>) -> Option<NamedItem> {
    if has_enough_materials(recipe, materials) == false {
        return None;
    }

    return None;
}

pub fn has_enough_materials(recipe: &Recipe, materials: &Vec<NamedItem>) -> bool {
    for item in recipe.items.iter() {
        let id = &item.material_id;

    }

    return true;
}

#[macro_export]
macro_rules! str {
    ($value:expr) => (
        String::from($value)
    )
}

#[cfg(test)]
mod tests {
    use crate::named_item::NamedItem;
    use crate::recipe_item::RecipeItem;
    use crate::recipe::Recipe;
    use crate::inventory_item::InventoryItem;
    use crate::inventory::Inventory;
    use std::collections::HashMap;
    use crate::Materials;
    use crate::material::Material;

    #[test]
    fn create_materials() {
        let mut materials = Materials::new();
        materials.insert(str!("wood"), Material::new(str!("wood"), str!("Wood"), 100.0));
        materials.insert(str!("steel"), Material::new(str!("steel"), str!("Steel"), 100.0));
        materials.insert(str!("spike"), Material::new(str!("spike"), str!("Spike"), 100.0));




        // let mut materials = Materials::new();
        // materials.insert(String::from("wood"), NamedItem::new(String::from("wood"), String::from("Wood"), None));
        // materials.insert(String::from("steel"), NamedItem::new(String::from("steel"), String::from("Steel"), None));
        // materials.insert(String::from("spike"), NamedItem::new(String::from("spike"), String::from("Spike"), None));
        // let materials_inventory: Vec<NamedItem> = vec![
        //     NamedItem::new(String::from("wood"), "Wood".into()),
        //     NamedItem::new(String::from("steel"), "Steel".into()),
        //     NamedItem::new(String::from("spike"), "Spike".into())
        // ];
        // let products: Vec<NamedItem> = vec![
        //     NamedItem::new(String::from("simple axe"), "Axe".into())
        // ];
        //
        // let axe_recipe = Recipe::new(String::from("simple axe"), "Simple Axe".into(), vec![
        //     RecipeItem::new(String::from("wood"), 10.0),
        //     RecipeItem::new(String::from("steel"), 3.0),
        //     RecipeItem::new(String::from("spike"), 1.0)
        // ]);
        //
        // let mut recipes = HashMap::new();
        // recipes.insert(String::from("simple axe"), axe_recipe);
        //
        // let inventory = Inventory::new(String::from("bag 1"), Vec::new());


    }
}

