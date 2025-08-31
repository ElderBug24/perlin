pub mod tools;

pub use crate::tools::*;

use rand::Rng;


fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

pub fn default_layers(depth: u32, falloff: f64) -> Vec<[f64; 2]> {
    return (0..depth)
        .map(|n| [(1.0/falloff).powi(n as i32), falloff.powi(n as i32)])
        .collect::<Vec<[f64; 2]>>();
}

pub struct PerlinNoiseMap {
    vector_map: Cache<Vec<isize>, Vec<f64>, usize>,
    cartesian_products_cache: Cache<usize, Vec<Vec<u8>>, ()>
}

impl PerlinNoiseMap {
    pub fn new() -> Self {
        return Self {
            vector_map: Cache::new(|vec: Vec<isize>, len: usize| -> Vec<f64> { new_rand_vec(len) }),
            cartesian_products_cache: Cache::new(|n: usize, _| -> Vec<Vec<u8>> { cartesian_products(n) })
        };
    }

    pub fn with_capacity(capacity: usize) -> Self {
        return Self {
            vector_map: Cache::with_capacity(|vec: Vec<isize>, len: usize| -> Vec<f64> { new_rand_vec(len) }, capacity),
            cartesian_products_cache: Cache::new(|n: usize, _| -> Vec<Vec<u8>> { cartesian_products(n) })
        };
    }

    pub fn show(&self) {
        println!("{:#?}", self.vector_map);
        println!("{:#?}", self.cartesian_products_cache);
    }

    pub fn get_vector(&mut self, pos: &Vec<isize>) -> &Vec<f64> {
        let mut rng = rand::rng();
        let reduced_pos = reduce_vec::<isize>(pos.clone());
        let v = self.vector_map.get(reduced_pos, pos.len());

        if v.len() < pos.len() {
            while v.len() < pos.len() {
                v.push(rng.random_range(-1.0..1.0));
            }
        }

        return &*v;
    }

    pub fn get(&mut self, pos: &Vec<f64>) -> f64 {
        let len = pos.len();

        let cpos: Vec<isize> = pos
            .iter()
            // .map(|n| *n as isize)
            // .map(|n| if *n < 0.0 { (*n as isize -1) as isize } else { *n as isize })
            // .map(|n| *n as isize - (*n < 0.0) as isize)
            .map(|n| n.floor() as isize)
            .collect();

        let rpos: Vec<f64> = pos
            .iter()
            .zip(&cpos)
            .map(|(n, rn)| *n - *rn as f64)
            .collect();

        let corners = self.cartesian_products_cache.get(len, ()).clone();

        let values: Vec<Vec<f64>> = corners
            .iter()
            .map(|p| self.get_vector(&p
                    .iter()
                    .zip(&cpos)
                    .map(|(p, cp)| *p as isize + *cp as isize)
                    .collect::<Vec<isize>>())
                .clone())
            .collect();

        let pos_: Vec<Vec<f64>> = corners
            .iter()
            .map(|p| p
                    .iter()
                    .zip(&rpos)
                    .map(|(p, rp)| *p as f64 - *rp)
                    .collect())
            .collect();

        let values: Vec<f64> = values
            .iter()
            .zip(&pos_)
            .map(|(v, p)| v
                    .iter()
                    .zip(p)
                    .map(|(v, p)| *v * *p)
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

pub struct NoiseMap {
    layers: Vec<[f64; 2]>,
    perlin_noise_maps: Vec<PerlinNoiseMap>,
    total_coeff: f64
}

impl NoiseMap {
    pub fn new(layers: Vec<[f64; 2]>) -> Self {
        let perlin_noise_maps: Vec<PerlinNoiseMap> = (0..layers.len())
            .map(|_| PerlinNoiseMap::new())
            .collect();

        let total_coeff = layers
            .iter()
            .map(|layer| layer[1])
            .sum::<f64>();

        return Self {
            layers: layers,
            perlin_noise_maps: perlin_noise_maps,
            total_coeff: total_coeff
        };
    }

    pub fn with_capacity(layers: Vec<[f64; 2]>, capacity: usize) -> Self {
        let perlin_noise_maps: Vec<PerlinNoiseMap> = (0..layers.len())
            .map(|_| PerlinNoiseMap::with_capacity(capacity))
            .collect();

        let total_coeff = layers
            .iter()
            .map(|layer| layer[1])
            .sum::<f64>();

        return Self {
            layers: layers,
            perlin_noise_maps: perlin_noise_maps,
            total_coeff: total_coeff
        };
    }

    pub fn get(&mut self, pos: &Vec<f64>) -> f64 {
        let result = self.perlin_noise_maps
            .iter_mut()
            .zip(&self.layers)
            .map(|(map, layer)| map.get(&pos
                    .iter()
                    .map(|n| *n * layer[0])
                    .collect())
                * layer[1])
            .sum::<f64>();

        return result / self.total_coeff;
    }

    pub fn show(&self) {
        for perlin_noise_map in &self.perlin_noise_maps {
            perlin_noise_map.show();
        }
    }
}

