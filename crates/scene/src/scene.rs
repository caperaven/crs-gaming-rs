use crate::DivisionType;
use crate::cell::Cell;

pub struct Scene {
    division_type: DivisionType,
    cells: Vec<Cell>
}

impl Scene {
    pub fn new(division_type: DivisionType) -> Scene {
        Scene {
            division_type,
            cells: vec![]
        }
    }

    pub fn add(&self) {
        match self.division_type {
            DivisionType::Quadtree => {
                sub_divide_quadtree(self)
            }
            DivisionType::Octree => {
                sub_divide_octree(self);
            }
        }
    }
}

fn sub_divide_quadtree(scene: &Scene) {

}

fn sub_divide_octree(scene: &Scene) {

}

#[cfg(test)]
mod tests {
    use crate::scene::Scene;
    use crate::DivisionType;

    #[test]
    fn scene_create_test() {
        let scene = Scene::new(DivisionType::Octree);

        assert_eq!(true, true);
    }
}