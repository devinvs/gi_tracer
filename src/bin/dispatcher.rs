use gi_tracer::world::World;
use gi_tracer::vector::Vec3;
use gi_tracer::camera::Camera;
use gi_tracer::material::{Material, Color, Light, Texture};
use gi_tracer::job::RenderJob;
use gi_tracer::ply::load_ply;

use std::net::TcpStream;
use std::io::{Write, Read};
use std::sync::Arc;

use serde_binary::{to_vec, from_vec};
use serde_binary::binary_stream::Endian;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const SAMPLES: usize = 100;

const CLIENTS: [&'static str; 5] = [
    "glados.cs.rit.edu:8000",
    "silver.cs.rit.edu:8000",
    "queeg.cs.rit.edu:8000",
    "neon.cs.rit.edu:8000",
    "argon.cs.rit.edu:8000"
];

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
    // Setup the scene
    let camera = Arc::new(Camera::new(
        Vec3::new(0.0, 0.0, 0.2),
        Vec3::new(0.0, 0.1, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        120.0,
        WIDTH as f32 / HEIGHT as f32,
        0.5
    ));

    let mut world = World::new();

    let mat = world.add_material(Material::Phong(Texture::Solid(Color::RGB(200, 76, 40)), 0.6, 0.3, 5.1, 0.0, 0.0));

    world.lights.push(Light {
        pos: Vec3::new(0.0, 1.0, 2.0),
        color: Vec3::new(0.5, 0.5, 0.5)
    });

    for t in load_ply("./bunny/reconstruction/bun_zipper.ply") {
        world.add_entity(t, mat);
    }

    let world = Arc::new(world);

    // Create the render jobs based on the number of clients
    let mut jobs = vec![];

    for i in 0..CLIENTS.len() {
        jobs.push(RenderJob {
            camera: camera.clone(),
            world: world.clone(),
            img_width: WIDTH,
            img_height: HEIGHT,
            num_samples: SAMPLES,
            start: i*WIDTH*HEIGHT/CLIENTS.len(),
            count: WIDTH*HEIGHT/CLIENTS.len(),
        });
    }

    let jobs = Arc::new(jobs);

    // For each client spawn a thread to send out the render job and then return the response.
    let handles = CLIENTS.iter().enumerate()
        .map(|(i, client)| {
            let jobs_c = jobs.clone();

            (i, std::thread::spawn(move || {
                let mut stream = TcpStream::connect(client).unwrap();

                // Send the render job
                let serjob = to_vec(&jobs_c[i], Endian::Big).unwrap();

                // Write number of bytes in job
                stream.write(&usize::to_be_bytes(serjob.len())).unwrap();

                // Send render job
                stream.write_all(&to_vec(&jobs_c[i], Endian::Big).unwrap()).unwrap();

                // Receive the result
                let mut size = [0;8];
                stream.read_exact(&mut size).unwrap();
                let size = usize::from_be_bytes(size);

                let mut buf = vec![0; size];
                stream.read_exact(&mut buf).unwrap();

                let res: Vec<Vec3<f32>> = from_vec(buf, Endian::Big).unwrap();
                res
            }))
        }).collect::<Vec<_>>();

    let mut img = vec![Color::RGB(0,0,0); WIDTH*HEIGHT];

    handles.into_iter()
        .map(|(i, h)| {
            (i, h.join().unwrap())
        })
    .for_each(|(i, seg)| {
        let j = &jobs[i];

        img[j.start..j.start+j.count].copy_from_slice(&seg);
    });

    tone_map(&mut img);
    output_ppm(&img, WIDTH, HEIGHT);
}
