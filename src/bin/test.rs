use image::{Luma, ImageBuffer};

use perlin::*;


const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {
    let mut perlin_noise_map = PerlinNoiseMap::new();

    let mut img = ImageBuffer::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let result = perlin_noise_map.get(&vec![x as f64 / 64.0, y as f64 / 64.0]);
            let value = ((result + 0.8) / 0.8 / 2.0 * 256.0) as u8;
            img.put_pixel(x, y, Luma([value]));
        }
    }

    img.save("output.png").expect("Failed to save image");
}

