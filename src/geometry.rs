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
        -(point-self.center).normalized()
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
        -(self.v2-self.v0).cross(&(self.v1-self.v0)).normalized()
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

    pub fn fit(&self) -> AABB {
        match self {
            Geometry::Triangle(Triangle { v0, v1, v2 }) => {
                let minx = v0.x.min(v1.x).min(v2.x);
                let miny = v0.y.min(v1.y).min(v2.y);
                let minz = v0.z.min(v1.z).min(v2.z);

                let maxx = v0.x.max(v1.x).max(v2.x);
                let maxy = v0.y.max(v1.y).max(v2.y);
                let maxz = v0.z.max(v1.z).max(v2.z);

                AABB {
                    min: Vec3::new(minx, miny, minz),
                    max: Vec3::new(maxx, maxy, maxz)
                }
            }
            Geometry::Sphere(Sphere { center, radius }) => {
                let mut min = center.clone();
                let mut max = center.clone();

                let r2 = radius.powi(2);
                min.x -= r2;
                min.y -= r2;
                min.z -= r2;

                max.x += r2;
                max.y += r2;
                max.z += r2;

                AABB { min, max }
            }
        }
    }

    pub fn left_of(&self, axis: Axis, v: f32) -> bool {
        match self {
            Geometry::Triangle(Triangle { v0, v1, v2 }) => {
                match axis {
                    Axis::X => v0.x <= v || v1.x <= v || v2.x <= v,
                    Axis::Y => v0.y <= v || v1.y <= v || v2.y <= v,
                    Axis::Z => v0.z <= v || v1.z <= v || v2.z <= v,
                }
            }
            Geometry::Sphere(Sphere { center, radius }) => {
                let r2 = radius.powi(2);

                match axis {
                    Axis::X => center.x-r2 <= v,
                    Axis::Y => center.y-r2 <= v,
                    Axis::Z => center.z-r2 <= v
                }
            }
        }
    }

    pub fn right_of(&self, axis: Axis, v: f32) -> bool {
        match self {
            Geometry::Triangle(Triangle { v0, v1, v2 }) => {
                match axis {
                    Axis::X => v0.x >= v || v1.x >= v || v2.x >= v,
                    Axis::Y => v0.y >= v || v1.y >= v || v2.y >= v,
                    Axis::Z => v0.z >= v || v1.z >= v || v2.z >= v,
                }
            }
            Geometry::Sphere(Sphere { center, radius }) => {
                let r2 = radius.powi(2);

                match axis {
                    Axis::X => center.x+r2 >= v,
                    Axis::Y => center.y+r2 >= v,
                    Axis::Z => center.z+r2 >= v
                }
            }
        }
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Axis { X, Y, Z }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AABB {
    pub min: Vec3<f32>,
    pub max: Vec3<f32>
}

impl AABB {
    pub fn union(self, other: Self) -> Self {
        Self {
            min: Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z)
            ),
            max: Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z)
            )
        }
    }

    pub fn split(self, axis: Axis) -> (Self, Self, f32) {
        match axis {
            Axis::X => {
                let mid = (self.min.x + self.max.x) / 2.0;

                let l = AABB {
                    min: self.min,
                    max: Vec3::new(mid, self.max.y, self.max.z)
                };
                let r = AABB {
                    min: Vec3::new(mid, self.min.y, self.min.z),
                    max: self.max
                };

                (l, r, mid)
            }
            Axis::Y => {
                let mid = (self.min.y + self.max.y) / 2.0;

                let l = AABB {
                    min: self.min,
                    max: Vec3::new(self.max.x, mid, self.max.z)
                };
                let r = AABB {
                    min: Vec3::new(self.min.x, mid, self.min.z),
                    max: self.max
                };

                (l, r, mid)
            }
            Axis::Z => {
                let mid = (self.min.z + self.max.z) / 2.0;

                let l = AABB {
                    min: self.min,
                    max: Vec3::new(self.max.x, self.max.y, mid)
                };
                let r = AABB {
                    min: Vec3::new(self.min.x, self.min.y, mid),
                    max: self.max
                };

                (l, r, mid)
            }
        }
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;


        if ray.dir.x == 0.0 {
            if ray.origin.x < self.min.x || ray.origin.x > self.max.x {
                return false;
            }
        } else {
            let t1 = (self.min.x - ray.origin.x) / ray.dir.x;
            let t2 = (self.max.x - ray.origin.x) / ray.dir.x;
            let (t1, t2) = if t1 > t2 { (t2, t1) } else { (t1, t2) };
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);
            if tmin > tmax {
                return false;
            }
        }

        if ray.dir.y == 0.0 {
            if ray.origin.y < self.min.y || ray.origin.y > self.max.y {
                return false;
            }
        } else {
            let t1 = (self.min.y - ray.origin.y) / ray.dir.y;
            let t2 = (self.max.y - ray.origin.y) / ray.dir.y;
            let (t1, t2) = if t1 > t2 { (t2, t1) } else { (t1, t2) };
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);
            if tmin > tmax {
                return false;
            }
        }

        if ray.dir.z == 0.0 {
            if ray.origin.z < self.min.z || ray.origin.z > self.max.z {
                return false;
            }
        } else {
            let t1 = (self.min.z - ray.origin.z) / ray.dir.z;
            let t2 = (self.max.z - ray.origin.z) / ray.dir.z;
            let (t1, t2) = if t1 > t2 { (t2, t1) } else { (t1, t2) };
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);
            if tmin > tmax {
                return false;
            }
        }

        true
    }
}
