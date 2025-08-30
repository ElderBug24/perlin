pub mod tools;

pub use crate::tools::*;

use rand::Rng;


pub struct PerlinNoiseMap {
    vector_map: Cache<Vec<isize>, Vec<f64>>,
    cartesian_products_cache: Cache<usize, Vec<Vec<u8>>>
}

impl PerlinNoiseMap {
    pub fn new() -> Self {
        return Self{
            vector_map: Cache::new(|vec: Vec<isize>| -> Vec<f64> { new_rand_vec(reduce_vec::<isize>(vec).len()) }),
            cartesian_products_cache: Cache::new(|n: usize| -> Vec<Vec<u8>> { cartesian_products(n) })
        };
    }

    pub fn with_capacity(capacity: usize) -> Self {
        return Self {
            vector_map: Cache::with_capacity(|vec: Vec<isize>| -> Vec<f64> { new_rand_vec(reduce_vec::<isize>(vec).len()) }, capacity),
            cartesian_products_cache: Cache::new(|n: usize| -> Vec<Vec<u8>> { cartesian_products(n) })
        };
    }

    pub fn show(&self) {
        println!("{:#?}", self.vector_map);
    }

    pub fn get_vector(&mut self, pos: &Vec<isize>) -> &Vec<f64> {
        return {
            let mut rng = rand::rng();
            let v = self.vector_map.get(pos.clone());

            if v.len() < pos.len() {
                while v.len() < pos.len() {
                    v.push(rng.random_range(-1.0..1.0));
                }
            }

            &*v
        };
    }

    pub fn get(&mut self, pos: &Vec<f64>) -> f64 {
        let pos = reduce_vec::<f64>(pos.clone());
        let rpos = pos.iter().map(|n| *n % 1.0).collect::<Vec<f64>>();
        let cpos: Vec<usize> = {
            let mut v: Vec<usize> = Vec::with_capacity(pos.len());

            for c in pos {
                v.push(c as usize);
            }

            v
        };

        0f64
        // todo!();
    }
}

