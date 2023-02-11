use crate::vector::Vec3;

#[derive(Debug, Copy, Clone)]
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

pub trait Hittable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;   // (distance, point)
}

pub struct Sphere {
    center: Vec3<f32>,
    radius: f32,
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;  // Vector from ray origin to circle center
        let b = oc.dot(&ray.dir);           // cos of Angle between a center collision and the actual direction
        let c = oc.dot(&oc) - self.radius*self.radius;            // projection of ray onto center collision
        let h = b*b - c;

        if h < 0.0 {
            None
        } else {
            let h = h.sqrt();
            Some(-b-h)
        }
    }
}

pub struct Triangle {
    v0: Vec3<f32>,
    v1: Vec3<f32>,
    v2: Vec3<f32>,
}

impl Hittable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let v1v0 = self.v1 - self.v0;
        let v2v0 = self.v2 - self.v0;
        let rov0 = ray.origin - self.v0;
        let n = v1v0.cross(&v2v0);
        let q = rov0.cross(&ray.dir);
        let d = 1.0 / ray.dir.dot(&n);
        let u = d*(-q).dot(&v2v0);
        let v = d*q.dot(&v1v0);
        let t = d*(-n).dot(&rov0);

        if u<0.0 || v<0.0 || (u+v)>1.0 || t<0.0 {
            None
        } else {
            Some(t)
        }
    }
}

impl Hittable for Vec<Triangle> {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        self.iter()
            .filter_map(|t| t.intersect(&ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }
}

pub enum Geometry {
    Sphere(Sphere),
    Triangle(Triangle),
    TriangleList(Vec<Triangle>)
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

    pub fn new_floor(corner: Vec3<f32>, width: f32, height: f32) -> Self {
        Self::TriangleList(vec![
            Triangle {
                v0: corner,
                v1: Vec3::new(corner.x+width, corner.y, corner.z),
                v2: Vec3::new(corner.x, corner.y, corner.z+height),
            },
            Triangle {
                v0: Vec3::new(corner.x+width, corner.y, corner.z+height),
                v1: Vec3::new(corner.x+width, corner.y, corner.z),
                v2: Vec3::new(corner.x, corner.y, corner.z+height),
            }
        ])
    }
}

impl Hittable for Geometry {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        match self {
            Geometry::Sphere(s) => s.intersect(&ray),
            Geometry::Triangle(t) => t.intersect(&ray),
            Geometry::TriangleList(tl) => tl.intersect(&ray)
        }
    }
}
