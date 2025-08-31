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

        let cpos: Vec<isize> = pos
            .iter()
            .map(|n| *n as isize)
            .collect();

        let rpos: Vec<f64> = pos
            .iter()
            .zip(&cpos)
            .map(|(a, b)| *a - *b as f64)
            .collect();

        let corners = self.cartesian_products_cache.get(cpos.len());

        let values: Vec<Vec<f64>> = corners
            .iter()
            .map(|p| self.vector_map.get(p
                    .iter()
                    .zip(&cpos)
                    .map(|(a, b)| *a as isize + *b)
                    .collect()).clone())
            .collect();

        let pos_: Vec<Vec<f64>> = corners
            .iter()
            .map(|p| p
                    .iter()
                    .zip(&rpos)
                    .map(|(a, b)| *a as f64 - *b)
                    .collect())
            .collect();

        let values: Vec<f64> = values
            .iter()
            .zip(&pos_)
            .map(|(a, b)| a
                    .iter()
                    .zip(b)
                    .map(|(a, b)| *a * *b)
                    .sum())
            .collect();

        let rpos: Vec<f64> = rpos
            .iter()
            .map(|n| fade(*n))
            .collect();

        let result = flat_nd_lerp(&rpos, &corners, &values);

        return result;
    }
}

fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

