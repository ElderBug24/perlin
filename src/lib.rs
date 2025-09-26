pub mod tools;

pub use crate::tools::*;

use std::cmp::max;

use rand::Rng;


fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

pub fn default_layers(depth: u32, falloff: f64) -> Vec<[f64; 2]> {
    return (0..depth)
        .map(|n| [(1.0/falloff).powi(n as i32), falloff.powi(n as i32)])
        .collect::<Vec<[f64; 2]>>();
}

const PERLIN_NOISE_MAP_VECTOR_MAP_FUNC: fn(Vec<isize>, usize) -> Vec<f64> = |_vec: Vec<isize>, len: usize| -> Vec<f64> { new_rand_vec(len) };
const PERLIN_NOISE_MAP_CARTESIAN_PRODUCTS_FUNC: fn(usize, ()) -> Vec<Vec<u8>> = |n: usize, _| -> Vec<Vec<u8>> { cartesian_products(n) };

pub struct PerlinNoiseMap {
    vector_map: Cache<Vec<isize>, Vec<f64>, usize>,
    cartesian_products_cache: Cache<usize, Vec<Vec<u8>>, ()>
}

impl PerlinNoiseMap {
    pub fn new() -> Self {
        return Self {
            vector_map: Cache::new(PERLIN_NOISE_MAP_VECTOR_MAP_FUNC),
            cartesian_products_cache: Cache::new(PERLIN_NOISE_MAP_CARTESIAN_PRODUCTS_FUNC)
        };
    }

    pub fn with_capacity(capacity: usize) -> Self {
        return Self {
            vector_map: Cache::with_capacity(PERLIN_NOISE_MAP_VECTOR_MAP_FUNC, capacity),
            cartesian_products_cache: Cache::new(PERLIN_NOISE_MAP_CARTESIAN_PRODUCTS_FUNC)
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

        for _ in 0..(max(pos.len() - v.len(), 0)) {
            v.push(rng.random_range(-1.0..1.0));
        }

        return &*v;
    }

    pub fn get_vector_map(&mut self) -> &Cache<Vec<isize>, Vec<f64>, usize> {
        return &self.vector_map;
    }

    pub fn clear_vector_map(&mut self) {
        self.vector_map.clear();
    }

    pub fn remove_from_vector_map(&mut self, pos: Vec<isize>) -> Option<Vec<f64>> {
        return self.vector_map.remove(pos);
    }

    pub fn get(&mut self, pos: &Vec<f64>) -> f64 {
        let corners = self.cartesian_products_cache.get(pos.len(), ()).clone();

        let cpos: Vec<isize> = pos
            .iter()
            .map(|n| n.floor() as isize)
            .collect();

        let rpos: Vec<f64> = pos
            .into_iter()
            .zip(&cpos)
            .map(|(n, rn)| *n - *rn as f64)
            .collect();

        let values: Vec<Vec<f64>> = corners
            .iter()
            .map(|p| {
                self.get_vector(&p
                        .iter()
                        .zip(&cpos)
                        .map(|(p, cp)| *p as isize + *cp as isize)
                        .collect::<Vec<isize>>())
                    .clone()})
            .collect();

        let pos_: Vec<Vec<f64>> = corners
            .iter()
            .map(|p| {
                p
                    .iter()
                    .zip(&rpos)
                    .map(|(p, rp)| *p as f64 - *rp)
                    .collect()})
            .collect();

        let values: Vec<f64> = values
            .into_iter()
            .zip(&pos_)
            .map(|(v, p)| {
                v
                    .iter()
                    .zip(p)
                    .map(|(v, p)| *v * *p)
                    .sum()})
            .collect();

        let fpos: Vec<f64> = rpos
            .into_iter()
            .map(|n| fade(n))
            .collect();

        let result = flat_nd_lerp(&fpos, &corners, &values);

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
            .map(|(map, layer)| {
                map.get(&pos
                        .iter()
                        .map(|n| *n * layer[0])
                        .collect())
                    * layer[1]})
            .sum::<f64>();

        return result / self.total_coeff;
    }

    pub fn show(&self) {
        for perlin_noise_map in &self.perlin_noise_maps {
            perlin_noise_map.show();
        }
    }
}

fn render_memory_efficient(layers: Vec<[f64; 2]>, ranges: Vec<[f64; 3]>) -> Vec<Vec<f64>> {
    let mut noise_map = NoiseMap::new(layers);

    // pop ranges and generate iter for last dim so it gets used in the first iteration // other way around
    // let iter;
    // for range in ranges.into_iter().rev() {
    // }

    // return (y..ymax)
    //     .map(|y| {
    //         let y_ = y as f64 * ystep;
    //         let r = (x..xmax)
    //             .map(|x| {
    //                 let x_ = x as f64 * xstep;
    //                 noise_map.get(vec![x_, y_])})
    //             .collect::<Vec<f64>>();
    //         (x..xmax)
    //             .map(|x| {
    //                 noise_map.remove(vec![x.floor() - 1, y.floor() - 1])
    //             })
    //         return r;
    //         })
    //     .collect::<Vec<Vec<f64>>>();

    todo!();
}

