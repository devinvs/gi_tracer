# The Ray Tracer

Brought to you by my blood sweat and tears.

### Running

For normal operation the ppm values are written to stdout. So to render the
regular scene run:

```sh
cargo run --release > image.ppm
```

For running the advanced kdtree assignment also provide the path to the ply
file:

```sh
cargo run --release --bin bunny -- file.ply > image.ppm
```

The advanced tone map assignment is included in the main binary,
change the line at the bottom of `src/main.rs` from

```rust
tone_map(&mut img, Algorithm::Ward);
```

to:

```rust
tone_map(&mut img, Algorithm::ALM(0.85));
```

where 0.85 is the bias parameter.

For running the distributed rendering components more setup is involved.
The system consists of a single dispatcher and multiple workers.

On every machine that you want doing rendering work start the worker code
listening on your choice of tcp port:

```sh
cargo run --release --bin worker -- port
```

Then, modify `src/bin/dispatcher.rs` such that all of your worker machines are
present in `CLIENTS`.

Download the stanford bunny
(http://graphics.stanford.edu/pub/3Dscanrep/bunny.tar.gz) and extract it in the
root of the directory.

Finally, start the dispatcher thread:

```sh
cargo run --release --bin dispatcher > image.ppm
```

The dispatcher thread will send out jobs to each of the workers and piece
together the render.

### Notable Files

- src/main.rs
    + Entrypoint for normal running, defines scene and glues together all the steps
- src/world.rs
    + Struct that efficiently stores object, light, and material data
    + Also defines firing an arbitrary ray in the world
- src/camera.rs
    + Defines a perspective camera and a method to get a ray from the camera's origin to an x,y point on the image plane.
- src/vector.rs
    + defines a basic vector struct with operator overloading for convenicne
- src/geometry.rs
    + defines basic scene geometry for spheres, triangles, and rays, includes intersection code
- src/material.rs
    + defines texture and material data types, along with different shading
      functions for Normals, Distance, Phong, and CookTorrance
- src/tone_map.rs
    + defines tone mapping operators and functions
- src/bin/bunny.rs
    + Entrypoint for kdtree advanced assignment
- src/ply.rs
    + defines code for reading and loading ply files
- src/kdtree.rs
    + defines a kdtree and code for traversing it
- src/bin/dispatcher.rs
    + Entrypoint for dispatcher thread
- src/bin/worker.rs
    + Entrypoint for worker thread
- src/job.rs
    + Defines code for how a rendering job is defined
