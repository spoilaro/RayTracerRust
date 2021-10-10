mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;
use std::fs::File;
use std::io::prelude::*;


fn write_ppm(w: i32, h: i32){
    println!("P3\n{} {}\n255", w, h);

    for j in (0..h).rev(){
        eprintln!("Scanlines remaining: {}", j);
        for i in (0..w){
            let r = i as f32 / w as f32;
            let g = j as f32 / h as f32;
            let b: f32 = 0.2;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            println!("{} {} {}", ir ,ig, ib);
        }
    }
    eprintln!("Done!");
}

fn main() {
    let width: i32 = 256;
    let height: i32 = 256;

    write_ppm(width, height);

    let v1 = Vec3::new(1f32, 2f32, 6f32);
    let v2 = Vec3::new(2f32, 6f32, 8f32);

    let v3 = v1 + v2;
    println!("Added v1 and v2, result is {:?}", v3);
}
