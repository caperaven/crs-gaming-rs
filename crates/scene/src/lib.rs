mod cell;
mod scene;

pub enum DivisionType {
    Quadtree,
    Octree
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
