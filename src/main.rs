use gi_tracer::world::World;
use gi_tracer::vector::Vec3;
use gi_tracer::geometry::Geometry;
use gi_tracer::camera::Camera;
use gi_tracer::material::Color;

use rayon::prelude::*;

use rand::thread_rng;
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const SAMPLES: usize = 50;

fn output_ppm(img: &Vec<Vec3<f32>>, w: usize, h: usize) {
    // Header
    println!("P3");
    println!("{w} {h}");
    println!("255");

    // Now print data
    for row in 0..h {
        for col in 0..w {
            let color = img[row*w+col] * 255.0;
            println!("{} {} {}", color.x as u8, color.y as u8, color.z as u8);
        }
    }
}

fn main() {
    let cam = Camera::new(
        Vec3::new(0.0, 0.1, -1.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        WIDTH as f32 / HEIGHT as f32,
        1.0
    );

    let mut world = World::new();
    world.add_entity(
        Geometry::new_sphere(
            Vec3::new(0.0, 0.0, 5.0),
            1.0,
        ),
        Color::RGB(136, 55, 204)
    );

    world.add_entity(
        Geometry::new_sphere(
            Vec3::new(-1.5, -0.2, 8.0),
            1.0
        ),
        Color::RGB(136, 255, 104)
    );

    world.add_entity(
        Geometry::new_floor(
            Vec3::new(-2.0, -2.0, -2.0),
            2.2,
            10.0
        ),
        Color::RGB(89, 76, 40)
    );

    let x_jitter = 1.0 / WIDTH as f32 / 2.0;
    let y_jitter = 1.0 / WIDTH as f32 / 2.0;

    let mut img = vec![Color::RGB(0,0,0); WIDTH*HEIGHT];
    img.par_iter_mut()
        .enumerate()
        .for_each(|(i, p)| {
            let y = i / WIDTH;
            let x = i % WIDTH;

            *p = (0..SAMPLES).map(|_| {
                let cx = 1.0 - x as f32 / WIDTH as f32 + thread_rng().gen_range(-x_jitter..x_jitter);
                let cy = 1.0 - y as f32 / HEIGHT as f32 + thread_rng().gen_range(-y_jitter..y_jitter);

                let ray = cam.get_ray(cx, cy);
                if let Some((i, _)) = world.intersect(&ray) {
                    world.shade(i)
                } else {
                    Color::RGB(122, 138, 214)
                }
            }).sum::<Vec3<f32>>() / SAMPLES as f32;
        });

    output_ppm(&img, WIDTH, HEIGHT);
}

