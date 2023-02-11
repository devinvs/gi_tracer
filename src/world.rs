use crate::geometry::{Hittable, Geometry, Ray};
use crate::vector::Vec3;

pub struct World {
    pub geometry: Vec<Geometry>,
    pub color: Vec<Vec3<f32>>
}

impl World {
    pub fn new() -> Self {
        Self {
            geometry: Vec::new(),
            color: Vec::new()
        }
    }

    pub fn add_entity(&mut self, geometry: Geometry, color: Vec3<f32>) {
        self.geometry.push(geometry);
        self.color.push(color);
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(usize, f32)> {
        self.geometry.iter()
            .enumerate()
            .filter_map(|(i, g)| {
                g.intersect(ray).map(|d| (i, d))
            }).min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    pub fn shade(&self, id: usize) -> Vec3<f32> {
        self.color[id]
    }
}
