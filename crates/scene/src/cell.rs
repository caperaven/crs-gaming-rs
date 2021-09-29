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