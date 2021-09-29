use crate::cell::Cell;
use euclid::Point3D;

/// Scene management
/// Scene manages:
/// 1. What items make up the scene
/// 2. What items are visible
/// 3. Queries to support hit test detection features
///
/// It has a build in octree for object queries
/// This is just the scene data and does not manage the rendering
pub struct Scene {
    next_id: i64,
    cell: Cell
}

impl Scene {
    /// Create a new instance of scene
    pub fn new(x: f64, y: f64, z: f64, width: f64, height: f64, depth: f64) -> Scene {
        let p1 = Point3D::new(x, y, z);
        let p2 = Point3D::new(x + width, y + height, z + depth);

        Scene {
            next_id: 0,
            cell: Cell::new(p1, p2)
        }
    }
    
    /// Add a item to the scene
    pub fn add(&mut self) -> i64 {
        let id = self.next_id;
        self.next_id = id + 1;
        return id;
    }

    /// Remove from the scene an item with the id value provided
    pub fn remove(&mut self, _id: i64) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use crate::scene::*;

    #[test]
    fn scene_constructor() {
        let scene = Scene::new(-50., -50., -50., 100., 100., 100.);
        assert_eq!(scene.next_id, 0);
        assert_eq!(scene.cell.area.min.x, -50.);
        assert_eq!(scene.cell.area.min.y, -50.);
        assert_eq!(scene.cell.area.min.z, -50.);
        assert_eq!(scene.cell.area.max.x, 50.);
        assert_eq!(scene.cell.area.max.y, 50.);
        assert_eq!(scene.cell.area.max.z, 50.);
        assert_eq!(scene.cell.has_cells(), false);
    }

    #[test]
    fn scene_add() {
        let mut scene = Scene::new(-50., -50., -50., 100., 100., 100.);
        let id = scene.add();
        assert_eq!(id, 0);
        assert_eq!(scene.next_id, 1);
    }

    #[test]
    fn scene_remove() {
        let mut scene = Scene::new(-50., -50., -50., 100., 100., 100.);
        let success = scene.remove(1);
        assert_eq!(success, true);
    }
}
