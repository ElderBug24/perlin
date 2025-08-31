use image::{Luma, ImageBuffer};

use perlin::*;


const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn main() {
    let mut noise_map = NoiseMap::new(default_layers(4, 0.5));

    let mut img = ImageBuffer::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let (x_, y_) = (x as f64, y as f64);
            let (x_, y_) = (x_ - (WIDTH as f64/2 as f64), y_ - (HEIGHT as f64/2 as f64));
            let s = ((x_ * x_ + y_ * y_) as f64).sqrt() / 300.0;
            let result = noise_map.get(&vec![x_ as f64 / 64.0, y_ as f64 / 64.0]);
            let value = ((result + 0.8) / 0.8 / 2.0 * 256.0) as u8;
            img.put_pixel(x, y, Luma([value]));
        }
    }

    img.save("output.png").expect("Failed to save image");

    // noise_map.show();
}

