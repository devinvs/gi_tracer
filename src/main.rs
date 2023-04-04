use gi_tracer::world::World;
use gi_tracer::vector::Vec3;
use gi_tracer::geometry::Geometry;
use gi_tracer::camera::Camera;
use gi_tracer::material::{Material, Color, Light, Texture};

use rayon::prelude::*;

use rand::thread_rng;
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const SAMPLES: usize = 100;

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

fn tone_map(img: &mut Vec<Vec3<f32>>) {
    let max = img.iter()
        .map(|v| vec![v.x, v.y, v.z])
        .flatten()
        .max_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap();

    img.iter_mut()
        .for_each(|c| {
            c.x = c.x / max;
            c.y = c.y / max;
            c.z = c.z / max;
        })
}

fn main() {
    let cam = Camera::new(
        Vec3::new(0.0, 2.5, -1.0),
        Vec3::new(0.0, 2.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        120.0,
        WIDTH as f32 / HEIGHT as f32,
        0.5
    );

    let mut world = World::new();

    let mat1 = world.add_material(
        Material::Phong(
            Texture::Checker(
                Color::RGB(255, 0, 0),
                Color::RGB(0, 0, 255),
            )
            , 0.8, 0.9, 0.0, 0.0, 0.0)
    );

    /*
    let mat2 = world.add_material(
        Material::CookTorrance(Texture::Solid(Color::RGB(136, 55, 204)), 0.1, 0.06, 0.1)
    );

    let mat3 = world.add_material(
        Material::CookTorrance(Texture::Solid(Color::RGB(136, 255, 104)), 0.8, 0.1, 0.2)
    );*/

    let mat2 = world.add_material(Material::Phong(Texture::Solid(Color::RGB(200, 76, 40)), 0.8, 0.1, 0.1, 0.0, 0.0));
    let mat3 = world.add_material(Material::Phong(Texture::Solid(Color::RGB(200, 76, 40)), 0.8, 0.1, 0.1, 0.95, 0.0));

    world.add_floor(
        Vec3::new(-3.5, -1.0, -3.0),
        5.0,
        10.0,
        mat1
    );

    world.add_entity(
        Geometry::new_sphere(Vec3::new(0.0, 2.0, 1.0), 1.0),
        mat2
    );

    world.add_entity(
        Geometry::new_sphere(
            Vec3::new(-1.5, 1.0, 1.5),
            0.8
        ),
        mat3
    );

    world.lights.push(Light {
        pos: Vec3::new(-2.0, 5.0, -2.0),
        color: Vec3::new(0.5, 0.0, 0.0)
    });

    world.lights.push(Light {
        pos: Vec3::new(2.0, 5.0, -2.0),
        color: Vec3::new(0.0, 0.0, 0.5)
    });

    let x_jitter = 1.0 / WIDTH as f32 / 2.0;
    let y_jitter = 1.0 / WIDTH as f32 / 2.0;

    let mut img = vec![Color::RGB(0,0,0); WIDTH*HEIGHT];
    img.par_iter_mut()
        .enumerate()
        .for_each(|(i, p)| {
            let y = i / WIDTH;
            let x = i % WIDTH;

            *p = (0..SAMPLES).map(|_| {
                let cx = x as f32 / WIDTH as f32 + thread_rng().gen_range(-x_jitter..x_jitter);
                let cy = 1.0 - y as f32 / HEIGHT as f32 + thread_rng().gen_range(-y_jitter..y_jitter);

                let ray = cam.get_ray(cx, cy);
                world.fire(&ray)
            }).sum::<Vec3<f32>>() / SAMPLES as f32;
        });

    tone_map(&mut img);
    output_ppm(&img, WIDTH, HEIGHT);
}

