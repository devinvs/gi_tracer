use crate::vector::Vec3;

pub type Point = Vec3<f32>;

pub struct Color;
impl Color {
    #[allow(non_snake_case)]
    pub fn RGB(r: u8, g: u8, b: u8) -> Vec3<f32> {
        Vec3::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub dir: Vec3<f32>
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3<f32>) -> Self {
        Self { origin, dir: dir.normalized() }
    }

    pub fn from_points(a: Point, b: Point) -> Self {
        Self {
            origin: a,
            dir: (a-b).normalized()
        }
    }
}

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Vec3<f32>>;
}

pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
    pub color: Vec3<f32>
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Vec3<f32>> {
        let oc = ray.origin - self.center;
        let b = oc.dot(&ray.dir)*2.0;
        let c = oc.mag()*oc.mag() - self.radius*self.radius;
        let discriminant = b*b - 4.0*c;

        if discriminant <= 0.0 {
            None
        } else {
            Some(self.color)
        }
    }
}

pub struct Triangle {
    pub v0: Vec3<f32>,
    pub v1: Vec3<f32>,
    pub v2: Vec3<f32>,
    pub color: Vec3<f32>
}

impl Object for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Vec3<f32>> {
        let v1v0 = self.v1 - self.v0;
        let v2v0 = self.v2 - self.v0;
        let rov0 = ray.origin - self.v0;
        let n = v1v0.cross(&v2v0);
        let q = rov0.cross(&ray.dir);
        let d = 1.0 / ray.dir.dot(&n);
        let u = d*(-q).dot(&v2v0);
        let v = d*q.dot(&v1v0);
        //let t = d*(-n).dot(&rov0);

        if u<0.0 || v<0.0 || (u+v)>1.0 {
            None
        } else {
            Some(self.color)
        }
    }
}

impl Object for Vec<Triangle> {
    fn intersect(&self, ray: &Ray) -> Option<Vec3<f32>> {
        for t in self {
            if let Some(color) = t.intersect(&ray) {
                return Some(color);
            }
        }

        None
    }
}

pub struct Floor;
impl Floor {
    pub fn new(corner: Vec3<f32>, width: f32, height: f32, color: Vec3<f32>) -> Vec<Triangle> {
        vec![
            Triangle {
                v0: corner,
                v1: Vec3::new(corner.x+width, corner.y, corner.z),
                v2: Vec3::new(corner.x, corner.y, corner.z+height),
                color
            },
            Triangle {
                v0: Vec3::new(corner.x+width, corner.y, corner.z+height),
                v1: Vec3::new(corner.x+width, corner.y, corner.z),
                v2: Vec3::new(corner.x, corner.y, corner.z+height),
                color
            }
        ]
    }
}

pub type Scene = Vec<Box<dyn Object + Send + Sync>>;
impl Object for Scene {
    fn intersect(&self, ray: &Ray) -> Option<Vec3<f32>> {
        for e in self {
            if let Some(color) = e.intersect(ray) {
                return Some(color);
            }
        }
        None
    }
}
