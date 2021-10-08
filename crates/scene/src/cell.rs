/// CellItem that is stored in the cell
pub struct CellItem {
    id: String,
    point: Option<euclid::Point3D<f64, f64>>,
    bounds: Option<euclid::Box3D<f64, f64>>
}

/// Cell that contains items
pub struct Cell {
    bounds: euclid::Box3D<f64, f64>,
    items: Vec<CellItem>
}