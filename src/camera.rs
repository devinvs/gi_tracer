use crate::geometry::Ray;
use crate::vector::Vec3;

pub struct Camera {
    origin: Vec3<f32>,
    lower_left_corner: Vec3<f32>,
    horizontal: Vec3<f32>,
    vertical: Vec3<f32>
}

impl Camera {
    pub fn new(
        look_from: Vec3<f32>,
        look_at: Vec3<f32>,
        up: Vec3<f32>,
        fov: f32,
        aspect_ratio: f32,
        focal_length: f32,
    ) -> Self {
        let theta = fov.to_radians();
        let w = (theta/2.0).tan();
        let width = 2.0*w;
        let height = width / aspect_ratio;

        eprintln!("w: {width} h: {height}");

        let n = (look_from-look_at).normalized();
        let u = up.cross(&n).normalized();
        let v = n.cross(&u).normalized();

        let horizontal = -u * focal_length * width;
        let vertical = v * focal_length * height;
        let lower_left_corner = look_from - horizontal/2.0 - vertical/2.0 - n*focal_length;

        eprintln!("h: {} h: {}", horizontal.mag(), vertical.mag());

        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal*x + self.vertical*y - self.origin
        )
    }
}
