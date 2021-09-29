extern crate euclid;

use euclid::{Box3D, Point3D};

type Cells = Option<Box<Cell>>; 

pub struct Cell {
    pub area: Box3D<f64, f64>,
    pub cells: Cells
}

impl Cell {
    pub fn new(min: Point3D<f64, f64>, max: Point3D<f64, f64>) -> Cell {
        Cell {
            area: Box3D::new(min, max),
            cells: None
        }
    }

    pub fn has_cells(&self) -> bool {
        match self.cells {
            None => false,
            Some(_) => true
        }
    }
}

#[cfg(test)]
mod test {
    use super::euclid::Point3D;
    use crate::cell::Cell;

    #[test]
    fn cell_create_text() {
        let p1 = Point3D::new(-50., -50., -50.);
        let p2 = Point3D::new(50., 50., 50.);
        let cell = Cell::new(p1, p2);

        assert_eq!(cell.has_cells(), false);
        assert_eq!(cell.area.min.x, -50.);
        assert_eq!(cell.area.min.y, -50.);
        assert_eq!(cell.area.min.z, -50.);
        assert_eq!(cell.area.max.x, 50.);
        assert_eq!(cell.area.max.y, 50.);
        assert_eq!(cell.area.max.z, 50.);
    }
}