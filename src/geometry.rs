use crate::vector::Vec3;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vec3<f32>,
    pub dir: Vec3<f32>
}

impl Ray {
    pub fn new(origin: Vec3<f32>, dir: Vec3<f32>) -> Self {
        Self { origin, dir: dir.normalized() }
    }

    pub fn from_points(a: Vec3<f32>, b: Vec3<f32>) -> Self {
        Self {
            origin: a,
            dir: (a-b).normalized()
        }
    }
}

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<f32>;      // (distance, point)
    fn normal(&self, point: Vec3<f32>) -> Vec3<f32>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sphere {
    center: Vec3<f32>,
    radius: f32,
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;  // Vector from ray origin to circle center
        let b = oc.dot(&ray.dir);           // cos of Angle between a center collision and the actual direction
        let c = oc.dot(&oc) - self.radius*self.radius;            // projection of ray onto center collision
        let h = b*b - c;

        if h < 0.0 {
            None
        } else {
            let h = h.sqrt();

            if -b-h < 0.0 { None } else { Some(-b-h) }
        }
    }

    fn normal(&self, point: Vec3<f32>) -> Vec3<f32> {
        (point-self.center).normalized()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Triangle {
    v0: Vec3<f32>,
    v1: Vec3<f32>,
    v2: Vec3<f32>,
}

impl Object for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let v1v0 = self.v1 - self.v0;
        let v2v0 = self.v2 - self.v0;
        let rov0 = ray.origin - self.v0;
        let n = v1v0.cross(&v2v0);
        let q = rov0.cross(&ray.dir);
        let ang = ray.dir.dot(&n);
        if ang.abs() == 0.0 {
            return None;
        }

        let d = 1.0 / ang;
        let u = d*(-q).dot(&v2v0);
        let v = d*q.dot(&v1v0);
        let t = d*(-n).dot(&rov0);

        if u<0.0 || v<0.0 || (u+v)>1.0 || t<0.0 {
            None
        } else {
            Some(t)
        }
    }

    fn normal(&self, _point: Vec3<f32>) -> Vec3<f32> {
        (self.v2-self.v0).cross(&(self.v1-self.v0)).normalized()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Geometry {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Geometry {
    pub fn new_sphere(center: Vec3<f32>, radius: f32) -> Self {
        Self::Sphere(Sphere {
            center,
            radius
        })
    }

    pub fn new_triangle(v0: Vec3<f32>, v1: Vec3<f32>, v2: Vec3<f32>) -> Self {
        Self::Triangle(Triangle {
            v0,
            v1,
            v2
        })
    }
}

impl Object for Geometry {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        if let Some(d) = match self {
            Geometry::Sphere(s) => s.intersect(&ray),
            Geometry::Triangle(t) => t.intersect(&ray),
        } {
            if d > 0.000001 {
                return Some(d)
            }
        }

        None
    }

    fn normal(&self, point: Vec3<f32>) -> Vec3<f32> {
        match self {
            Geometry::Sphere(s) => s.normal(point),
            Geometry::Triangle(t) => t.normal(point)
        }
    }
}
