use perlin::*;


fn main() {
    let mut perlin_noise_map = PerlinNoiseMap::new();
    
    // perlinNoiseMap.get(&vec![0, 0]);
    perlin_noise_map.get(&vec![1.2, 1.0]);

    perlin_noise_map.show();

    // println!("{:?}", v);

    // let vec: Vec<f64> = new_rand_vec(7);
    //
    // println!("{:#?}", vec);
}

