/**
*   links: https://docs.rs/euclid/0.22.6/euclid/struct.Box2D.html
*/

use euclid::default::{Box2D, Point2D};

type Point = Point2D<f64>;

struct Cells<T> {
    top_left        : Box<QuadTreeNode<T>>,
    top_right       : Box<QuadTreeNode<T>>,
    bottom_left     : Box<QuadTreeNode<T>>,
    bottom_right    : Box<QuadTreeNode<T>>,
}

impl<T> Cells<T> {
    pub fn new(bounds: Box2D<f64>, capacity: i8) -> Cells<T> {
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

pub struct QuadTreeNode<T> {
    bounds      : Box2D<f64>,
    capacity    : i8,
    cells       : Option<Cells<T>>,
    data        : Vec<T>
}

impl<T> QuadTreeNode<T> {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, capacity: i8) -> QuadTreeNode<T> {
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
            cells: None,
            data: vec![]
        }
    }

    pub fn has_children(&self) -> bool {
        match self.cells {
            Some(_) => true,
            None => false
        }
    }

    pub fn add(&mut self, item: T) {
        self.data.push(item);
    }
}

pub fn subdivide<T>(instance: &mut QuadTreeNode<T>) {
    instance.cells = Some(Cells::new(instance.bounds, instance.capacity));
}

#[cfg(test)]
mod tests {
    use crate::node::{QuadTreeNode, subdivide, Point};
    use std::borrow::Borrow;

    #[test]
    fn create_test() {
        let instance = QuadTreeNode::<Point>::new(-110., -120., 110., 120., 4);
        assert_eq!(instance.bounds.min.x, -110.);
        assert_eq!(instance.bounds.min.y, -120.);
        assert_eq!(instance.bounds.max.x, 110.);
        assert_eq!(instance.bounds.max.y, 120.);
        assert_eq!(instance.capacity, 4);
        assert_eq!(instance.has_children(), false);
    }

    #[test]
    fn subdivide_test() {
        let mut instance = QuadTreeNode::<Point>::new(-100., -100., 100., 100., 4);
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

    #[test]
    fn add_test() {
        let mut instance = QuadTreeNode::<Point>::new(-110., -120., 110., 120., 4);
        instance.add(Point::new(10., 10.));

        assert_eq!(instance.data.len(), 1);
    }
}