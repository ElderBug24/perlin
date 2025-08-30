use perlin::*;


fn main() {
    let mut perlinNoiseMap = PerlinNoiseMap::new();
    
    // perlinNoiseMap.get(&vec![0, 0]);
    perlinNoiseMap.get(&vec![1.2, 1.0]);

    perlinNoiseMap.show();

    // println!("{:?}", v);

    // let vec: Vec<f64> = new_rand_vec(7);
    //
    // println!("{:#?}", vec);
}

