use crate::geometry::Geometry;

#[derive(Debug)]
pub enum Axis {X, Y, Z }

#[derive(Debug)]
pub enum KDNode {
    /// Decision Branch on Axis = f32, id for lef
    Branch(Axis, f32, Box<KDNode>, Box<KDNode>),
    Leaf(Vec<usize>)
}

impl KDNode {

}

