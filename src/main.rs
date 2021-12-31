#![warn(clippy::pedantic)]

use crate::scene::Sphere;
mod scene;
use num::Float;
use rand::Rng;
use scene::{pt, AmbientLight, DirectionalLight, PointLight, Scene};

fn main() {
    let mut obj_vec: Vec<Sphere> = vec![];

    let sphereOrig = pt(0.0, 0., 2.0);
    let NUM_SPHERES: f32 = 70.0;
    let mut rand = rand::thread_rng();
    for step in 0..(NUM_SPHERES as isize) {
        obj_vec.push(Sphere::new(
            sphereOrig.add(&pt(
                (rand.gen_range(0..20)-10) as f32 / 2.0,
                (rand.gen_range(0..20)-10) as f32 / 3.0,
                (rand.gen_range(0..20)) as f32 / 3.0,
            )),
            0.25,
            [(rand.gen_range(0..100) as f32 / 100.).into(), (rand.gen_range(0..100) as f32 / 100.).into(), (rand.gen_range(0..100) as f32 / 100.).into(), 1.],
        ))
    }

    let scene = Scene {
        camera: pt(0., 0., 0.),
        viewport: pt(0., 0., 1.),
        // objects: vec![
        //     Sphere::new(pt(-0.75, 0.75, 2.5), 0.2, [1.0, 1.0, 1.0, 1.]),
        //         Sphere::new(pt(-0.6, 0.6, 2.2), 0.02, [1.0, 0.1, 0.1, 1.]),

        //     Sphere::new(pt(0.75, 0.75, 2.5), 0.2, [1.0, 1.0, 1.0, 1.]),
        //         Sphere::new(pt(0.6, 0.6, 2.2), 0.02, [1.0, 0.1, 0.1, 1.]),

        //     Sphere::new(pt(0., -0.4, 2.5), 0.1, [1.0, 0.2, 0.2, 1.]),

        //     Sphere::new(pt(-0.18, -0.38, 2.5), 0.1, [1.0, 0.2, 0.2, 1.]),
        //     Sphere::new(pt(0.18, -0.38, 2.5), 0.1, [1.0, 0.2, 0.2, 1.]),

        //     Sphere::new(pt(-0.35, -0.34, 2.5), 0.1, [1.0, 0.2, 0.2, 1.]),
        //     Sphere::new(pt(0.35, -0.34, 2.5), 0.1, [1.0, 0.2, 0.2, 1.]),

        //     Sphere::new(pt(-0.5, -0.28, 2.5), 0.1, [1.0, 0.2, 0.2, 1.]),
        //     Sphere::new(pt(0.5, -0.28, 2.5), 0.1, [1.0, 0.2, 0.2, 1.]),

        // ],
        lights: vec![
            Box::new(AmbientLight { intensity: 0.2 }),
            // Box::new(DirectionalLight{intensity: 2.0, position: pt(0., 0., 4.), direction: pt(0., -1., 0.)}),
            Box::new(PointLight {
                intensity: 5.0,
                position: pt(0., 0., 4.),
            }),
        ],
        objects: obj_vec,
    };
    let img = scene.render();
    println!("3 / 3 | Saving image...");
    img.save_with_format("./output.png", ::image::ImageFormat::Png)
        .unwrap();
    println!("Done rendering!");
}
