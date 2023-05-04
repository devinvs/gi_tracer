use crate::geometry::{Object, Geometry, Ray};
use crate::vector::Vec3;
use crate::material::{Material, Light, Color};
use crate::kdtree::KDNode;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    // Component Vectors
    pub geometry: Vec<Geometry>,
    pub material: Vec<usize>,

    // Global resources
    pub lights: Vec<Light>,
    pub materials: Vec<Material>,

    // Indexes
    pub kdtree: Option<KDNode>
}

impl World {
    pub fn new() -> Self {
        Self {
            geometry: Vec::new(),
            material: Vec::new(),

            lights: Vec::new(),
            materials: Vec::new(),
            kdtree: None
        }
    }

    pub fn add_entity(&mut self, geometry: Geometry, material: usize) {
        self.geometry.push(geometry);
        self.material.push(material);
    }

    pub fn add_floor(&mut self, corner: Vec3<f32>, width: f32, height: f32, material: usize) {
        self.add_entity(
            Geometry::new_triangle(
                corner,
                Vec3::new(corner.x, corner.y, corner.z+height),
                Vec3::new(corner.x+width, corner.y, corner.z),
            ),
            material
        );

        self.add_entity(
            Geometry::new_triangle(
                Vec3::new(corner.x+width, corner.y, corner.z+height),
                Vec3::new(corner.x+width, corner.y, corner.z),
                Vec3::new(corner.x, corner.y, corner.z+height),
            ),
            material
        );
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light)
    }

    pub fn add_material(&mut self, material: Material) -> usize {
        let id = self.materials.len();

        self.materials.push(material);

        id
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(usize, f32)> {
        if let Some(kdtree) = self.kdtree.as_ref() {
            kdtree.intersect(ray, &self.geometry)
        } else {
            self.geometry.iter()
                .enumerate()
                .filter_map(|(i, g)| {
                    g.intersect(ray).map(|d| (i, d))
                }).min_by(|a, b| {
                    let res = a.1.partial_cmp(&b.1);
                    if res.is_none() {
                        eprintln!("a: {:?} b: {:?}", self.geometry[a.0], self.geometry[b.0]);
                        eprintln!("ray: {ray:?}");
                        eprintln!("a: {a:?} b: {b:?}");
                    }

                    res.unwrap()
                })

            }
    }

    pub fn shade(&self, id: usize, ray: &Ray, dist: f32, depth: usize) -> Vec3<f32> {
        let p = ray.origin + ray.dir*dist;
        let norm = self.geometry[id].normal(p);

        let mat_id = self.material[id];
        self.materials[mat_id].shade(
            ray,
            dist,
            &norm,
            self,
            depth 
        )
    }

    pub fn fire(&self, ray: &Ray, depth: usize) -> Vec3<f32> {
        self.intersect(&ray)
            .map(|(i, d)| self.shade(i, &ray, d, depth))
            .unwrap_or(Color::RGB(31, 176, 255))
    }
}
