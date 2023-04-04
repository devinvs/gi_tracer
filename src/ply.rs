use crate::geometry::Geometry;
use crate::vector::Vec3;

use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn load_ply(path: &str) -> Vec<Geometry> {
    let r = BufReader::new(File::open(path).unwrap());

    let mut lines = r.lines()
        .filter_map(|l| l.ok());

    // Find the number of vertices
    let vcount = lines.find(|s| {
        s.starts_with("element vertex")
    }).map(|s| {
        s.split(" ").nth(2).unwrap().parse::<usize>().unwrap()
    }).unwrap();

    // Find the number of faces
    let fcount = lines.find(|s| {
        s.starts_with("element face")
    }).map(|s| {
        s.split(" ").nth(2).unwrap().parse::<usize>().unwrap()
    }).unwrap();

    // Consume the rest of the header
    lines.find(|s| s.starts_with("end_header")).unwrap();

    // Read in the list of all the vertices
    let mut vs = Vec::with_capacity(vcount);

    for _ in 0..vcount {
        let s = lines.next().unwrap();

        let mut ps = s.split_ascii_whitespace();
        let x = ps.next().unwrap().parse::<f32>().unwrap();
        let y = ps.next().unwrap().parse::<f32>().unwrap();
        let z = ps.next().unwrap().parse::<f32>().unwrap();

        vs.push(Vec3{ x, y, z });
    }

    // Now go through the faces and push all the triangles
    let mut ts = Vec::with_capacity(fcount);

    for _ in 0..fcount {
        let s = lines.next().unwrap();
        
        let mut ps = s.split_ascii_whitespace();
        ps.next().unwrap();

        let x = ps.next().unwrap().parse::<usize>().unwrap();
        let y = ps.next().unwrap().parse::<usize>().unwrap();
        let z = ps.next().unwrap().parse::<usize>().unwrap();

        ts.push(Geometry::new_triangle(vs[x], vs[y], vs[z]));
    }

    ts
}
