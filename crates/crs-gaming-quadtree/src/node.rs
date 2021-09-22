/**
*   links: https://docs.rs/euclid/0.22.6/euclid/struct.Box2D.html
*/

use euclid::default::{Box2D, Point2D};

struct Cells {
    top_left        : Box<QuadTreeNode>,
    top_right       : Box<QuadTreeNode>,
    bottom_left     : Box<QuadTreeNode>,
    bottom_right    : Box<QuadTreeNode>,
}

impl Cells {
    pub fn new(bounds: Box2D<f64>, capacity: i8) -> Cells {
        let mid_x = bounds.min.x + ((bounds.max.x - bounds.min.x) / 2.);
        let mid_y = bounds.min.y + ((bounds.max.y - bounds.min.y) / 2.);

        Cells {
            top_left:     Box::new(QuadTreeNode::new(bounds.min.x,  bounds.min.y,   mid_x,          mid_y,          capacity)),
            top_right:    Box::new(QuadTreeNode::new(mid_x,         bounds.min.y,   bounds.max.x,   mid_y,          capacity)),
            bottom_left:  Box::new(QuadTreeNode::new(bounds.min.x,  mid_y,          mid_x,          bounds.max.y,   capacity)),
            bottom_right: Box::new(QuadTreeNode::new(mid_x,         mid_y,          bounds.max.x,   bounds.max.y,   capacity))
        }
    }
}

pub struct QuadTreeNode {
    bounds      : Box2D<f64>,
    capacity    : i8,
    cells       : Option<Cells>
}

impl QuadTreeNode {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, capacity: i8) -> QuadTreeNode {
        QuadTreeNode {
            bounds: Box2D { min: Point2D {
                x: x1,
                y: y1,
                _unit: Default::default()
            }, max: Point2D {
                x: x2,
                y: y2,
                _unit: Default::default()
            } },

            capacity,
            cells: None
        }
    }

    pub fn has_children(&self) -> bool {
        match self.cells {
            Some(_) => true,
            None => false
        }
    }

    // pub fn add() {
    // }
}

pub fn subdivide(instance: &mut QuadTreeNode) {
    instance.cells = Some(Cells::new(instance.bounds, instance.capacity));
}

#[cfg(test)]
mod tests {
    use crate::node::{QuadTreeNode, subdivide};
    use std::borrow::Borrow;

    #[test]
    fn create_test() {
        let instance = QuadTreeNode::new(-110., -120., 110., 120., 4);
        assert_eq!(instance.bounds.min.x, -110.);
        assert_eq!(instance.bounds.min.y, -120.);
        assert_eq!(instance.bounds.max.x, 110.);
        assert_eq!(instance.bounds.max.y, 120.);
        assert_eq!(instance.capacity, 4);
        assert_eq!(instance.has_children(), false);
    }

    #[test]
    fn subdivide_test() {
        let mut instance = QuadTreeNode::new(-100., -100., 100., 100., 4);
        subdivide(&mut instance);

        assert_eq!(instance.has_children(), true);

        let top_left        = instance.cells.borrow().as_ref().unwrap().top_left.bounds;
        let top_right       = instance.cells.borrow().as_ref().unwrap().top_right.bounds;
        let bottom_left     = instance.cells.borrow().as_ref().unwrap().bottom_left.bounds;
        let bottom_right    = instance.cells.borrow().as_ref().unwrap().bottom_right.bounds;

        assert_eq!(top_left.min.x, -100.);
        assert_eq!(top_left.min.y, -100.);
        assert_eq!(top_left.max.x, 0.);
        assert_eq!(top_left.max.y, 0.);

        assert_eq!(top_right.min.x, 0.);
        assert_eq!(top_right.min.y, -100.);
        assert_eq!(top_right.max.x, 100.);
        assert_eq!(top_right.max.y, 0.);

        assert_eq!(bottom_left.min.x, -100.);
        assert_eq!(bottom_left.min.y, 0.);
        assert_eq!(bottom_left.max.x, 0.);
        assert_eq!(bottom_left.max.y, 100.);

        assert_eq!(bottom_right.min.x, 0.);
        assert_eq!(bottom_right.min.y, 0.);
        assert_eq!(bottom_right.max.x, 100.);
        assert_eq!(bottom_right.max.y, 100.);
    }
}