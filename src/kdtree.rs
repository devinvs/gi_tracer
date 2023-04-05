use crate::geometry::{Geometry, AABB, Axis, Ray, Object};
use crate::vector::Vec3;

use serde::{Serialize, Deserialize};

const MAX_DEPTH: usize = 20;
const NUM_POLYGONS: usize = 3;

#[derive(Debug, Serialize, Deserialize)]
pub enum KDNode {
    /// Decision Branch on Axis = f32, id for lef
    Branch(Axis, f32, Box<KDNode>, Box<KDNode>),
    Leaf(Vec<usize>)
}

impl KDNode {
    pub fn intersect(&self, r: &Ray, gs: &Vec<Geometry>) -> Option<(usize, f32)> {
        match self {
            KDNode::Branch(a, v, left, right) => {
                let (dist, dir) = match a {
                    Axis::X => (r.origin.x-v, r.dir.x),
                    Axis::Y => (r.origin.y-v, r.dir.y),
                    Axis::Z => (r.origin.z-v, r.dir.z)
                };

                match (dist < 0.0, dir < 0.0) {
                    // Only need to check left side
                    (true, true) => left.intersect(r, gs),
                    // Only need to check right side
                    (false, false) => right.intersect(r, gs),
                    // Check left and then right
                    (true, false) => {
                        let res = left.intersect(r, gs);
                        if res.is_none() {
                            right.intersect(r, gs)
                        } else {
                            res
                        }
                    }
                    // Check right and then left
                    (false, true) => {
                        let res = left.intersect(r, gs);
                        if res.is_none() {
                            right.intersect(r, gs)
                        } else {
                            res
                        }
                    }
                }
            }
            KDNode::Leaf(objs) => {
                objs.iter()
                    .filter_map(|&i| gs[i].intersect(r).map(|d| (i, d)))
                    .min_by(|a, b| {
                        a.1.partial_cmp(&b.1).unwrap()
                    })
            }
        }
    }
}

pub fn build_kdtree(g: &Vec<Geometry>) -> KDNode {
    let aabb = g.iter()
        .fold(
            AABB { min: Vec3::new(0.0, 0.0, 0.0), max: Vec3::new(0.0, 0.0, 0.0) },
            |a, b| a.union(b.fit())
        );

    build_kdtree_h(g.iter().enumerate().collect(), aabb, Axis::X, 0)
}

fn build_kdtree_h<'a>(g: Vec<(usize, &'a Geometry)>, aabb: AABB, axis: Axis, depth: usize) -> KDNode {
    // If we have reached our max depth return a leaf node containing the rest of the geometry
    if depth >= MAX_DEPTH {
        return KDNode::Leaf(g.iter().map(|a| a.0).collect());
    }

    // If there are few enough polygons also just create a leaf node
    if g.len() <= NUM_POLYGONS {
        return KDNode::Leaf(g.iter().map(|a| a.0).collect());
    }

    // Now just subdivide by the axis and recur
    let (l, r, d) = aabb.split(axis);

    let left = g.iter().filter(|(_, g)| g.left_of(axis, d)).map(|a| *a).collect();
    let right = g.iter().filter(|(_, g)| g.right_of(axis, d)).map(|a| *a).collect();

    let new_axis = match axis {
        Axis::X => Axis::Y,
        Axis::Y => Axis::Z,
        Axis::Z => Axis::X
    };

    KDNode::Branch(
        axis,
        d,
        Box::new(build_kdtree_h(left, l, new_axis, depth+1)),
        Box::new(build_kdtree_h(right, r, new_axis, depth+1))
    )
}
