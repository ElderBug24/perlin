use std::time::Instant;

use image::{Luma, ImageBuffer};
use rand::Rng;

use perlin::*;


const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

// const R: f64 = 2f64.sqrt() / 2.0;
// const R: f64 = 0.8;

fn main() {
    let R = 2f64.sqrt() / 2.0;
    // let mut noise_map = NoiseMap::new(default_layers(4, 0.5));
    //
    // let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    //
    // let start = Instant::now();
    //
    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         let (x_, y_) = (x as f64, y as f64);
    //         let (x_, y_) = (x_ - (WIDTH as f64/2.0), y_ - (HEIGHT as f64/2.0));
    //         let s = (x_ * x_ + y_ * y_).sqrt() / 60.0;
    //         let result = noise_map.get(&vec![x_ as f64 / 64.0, y_ as f64 / 64.0, s]);
    //         let value = ((result + R) / R / 2.0 * 256.0) as u8;
    //         img.put_pixel(x, y, Luma([value]));
    //     }
    // }
    // img.save("output.png").expect("Failed to save image");
    //
    // // noise_map.show();
    //
    // println!("{:?}", start.elapsed());

    const width: usize =  512;
    const height: usize = 512;
    const resx: usize = 64;
    const resy: usize = 64;

    let mut vectors = vec![0.0f64; (width+1)*(height+1)*2];
    let mut rng = rand::rng();
    for i in 0..vectors.len() {
        vectors[i] = rng.random_range(-1.0..1.0);
    }
    // println!("{:#?}", vectors);

    let mut output = vec![0.0; width*resx * height*resy];
    let start = Instant::now();
    render_arr_2d::<f64>(&vectors, &mut output, (width, height), (resx, resy), 1.0);
    println!("{:?}", start.elapsed());

    // println!("{:#?}", output);

    let mut img = ImageBuffer::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let value = ((output[x+y*width*resx] + R) / R / 2.0 * 256.0) as u8;
            img.put_pixel(x as u32, y as u32, Luma([value]));
        }
    }
    img.save("output2.png").unwrap();
}

