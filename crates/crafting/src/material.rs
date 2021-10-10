pub struct Material {
    pub id: String,
    pub name: String,
    pub quantity: f64
}

impl Material {
    pub fn new(id: String, name: String, quantity: f64) -> Material {
        Material {
            id,
            name,
            quantity
        }
    }

    pub fn set_quantity(&mut self, quantity: f64) {
        self.quantity = quantity;
    }
}

#[cfg(test)]
mod test {
    use crate::material::Material;

    #[test]
    fn material_constructor_test() {
        let material = Material::new(String::from("wood"), String::from("Wood"), 0.0);
        assert_eq!(material.id, "wood");
        assert_eq!(material.name, "Wood");
        assert_eq!(material.quantity, 0.0);
    }

    #[test]
    fn material_with_quantity_test() {
        let mut material = Material::new(String::from("wood"), String::from("Wood"), 0.0);
        material.set_quantity(100.0);

        assert_eq!(material.id, "wood");
        assert_eq!(material.name, "Wood");
        assert_eq!(material.quantity, 100.0);
    }
}