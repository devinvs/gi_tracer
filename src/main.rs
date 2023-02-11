use gi_tracer::vector::Vec3;
use gi_tracer::geometry::{Scene, Object, Color, Sphere, Floor};
use gi_tracer::camera::Camera;

use rayon::prelude::*;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn output_ppm(img: &Vec<Vec3<u8>>, w: usize, h: usize) {
    // Header
    println!("P3");
    println!("{w} {h}");
    println!("255");

    // Now print data
    for row in 0..h {
        for col in 0..w {
            let color = img[row*w+col];
            println!("{} {} {}", color.x, color.y, color.z);
        }
    }
}

fn main() {
    let cam = Camera::new(
        Vec3::new(0.3, 0.05, -0.4),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        WIDTH as f32 / HEIGHT as f32,
        1.0
    );

    let scene: Scene = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.3, 5.0),
            radius: 1.0,
            color: Color::new(136, 55, 204)
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.6, -0.2, 8.0),
            radius: 0.9,
            color: Color::new(70, 191, 128)
        }),
        Box::new(Floor::new(Vec3::new(-2.0, 0.0, -2.0),
            2.2,
            10.0,
            Color::new(89, 76, 40)
        ))
    ];

    let mut img = vec![Color::new(0,0,0); WIDTH*HEIGHT];
    img.par_iter_mut()
        .enumerate()
        .for_each(|(i, p)| {
            let y = i / WIDTH;
            let x = i % WIDTH;

            let ray = cam.get_ray(1.0-x as f32 / WIDTH as f32, 1.0-y as f32 / HEIGHT as f32);
            *p = scene.intersect(&ray).unwrap_or(Color::new(122, 138, 214));
        });

    output_ppm(&img, WIDTH, HEIGHT);
}

