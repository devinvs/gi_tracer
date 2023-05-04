use gi_tracer::vector::Vec3;
use gi_tracer::material::Color;
use gi_tracer::job::RenderJob;

use std::io::{BufReader, Read, Write};

use std::net::TcpListener;

use rayon::prelude::*;

use rand::thread_rng;
use rand::Rng;

use serde_binary::{from_vec, to_vec};
use serde_binary::binary_stream::Endian;

fn main() {
    let port = std::env::args().nth(1).unwrap();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    for stream in listener.incoming() {
        eprintln!("Received job!");
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(&mut stream);

        let mut size = [0; 8];
        reader.read_exact(&mut size).unwrap();
        let size = usize::from_be_bytes(size);

        let mut buf = vec![0; size];
        reader.read_exact(&mut buf).unwrap();

        let job: RenderJob = from_vec(buf, Endian::Big).unwrap();
        eprintln!("Render pixels {} to {}", job.start, job.start+job.count);

        let x_jitter = 1.0 / job.img_width as f32 / 2.0;
        let y_jitter = 1.0 / job.img_height as f32 / 2.0;

        let mut img = vec![Color::RGB(0,0,0); job.count];

        img.par_iter_mut()
            .enumerate()
            .for_each(|(i, p)| {
                let i = job.start + i;
                let y = i / job.img_width;
                let x = i % job.img_height;

                *p = (0..job.num_samples).map(|_| {
                    let cx = x as f32 / job.img_width as f32 + thread_rng().gen_range(-x_jitter..x_jitter);
                    let cy = 1.0 - y as f32 / job.img_height as f32 + thread_rng().gen_range(-y_jitter..y_jitter);

                    let ray = job.camera.get_ray(cx, cy);
                    job.world.fire(&ray, 0)
                }).sum::<Vec3<f32>>() / job.num_samples as f32;
            });
        
        eprintln!("Writing result!");

        let payload = to_vec(&img, Endian::Big).unwrap();
        stream.write(&usize::to_be_bytes(payload.len())).unwrap();

        stream.write_all(&payload).unwrap();
        eprintln!("Finished");
    }
}

