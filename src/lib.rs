use std::collections::HashMap;
use std::hash::Hash;

use rand::Rng;
use num::Num;


pub struct PerlinNoiseMap {
    vector_map: Cache<Vec<isize>, Vec<f64>>,
    // cartesian_products_cache: Cache<usize, Vec<Vec<u8>>, fn(Vec<isize>) -> Vec<f64>>
}

impl PerlinNoiseMap {
    pub fn new() -> Self {
        return Self{
            vector_map: Cache::new(|vec: Vec<isize>| -> Vec<f64> { new_rand_vec(reduce_vec::<isize>(vec).len()) })
        };
    }

    pub fn with_capacity(capacity: usize) -> Self {
        return Self {
            vector_map: Cache::with_capacity(|vec: Vec<isize>| -> Vec<f64> { new_rand_vec(reduce_vec::<isize>(vec).len()) }, capacity)
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
        let _cpos: Vec<usize> = {
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

#[derive(Debug)]
struct Cache<K: Eq + Hash + Clone, V> {
    map: HashMap<K, V>,
    func: fn(K) -> V
}

impl<K: Eq + Hash + Clone, V> Cache<K, V> {
    fn new(func: fn(K) -> V) -> Self {
        return Self {
            map: HashMap::new(),
            func: func
        };
    }

    fn with_capacity(func: fn(K) -> V, size: usize) -> Self {
        return Self {
            map: HashMap::with_capacity(size),
            func: func
        };
    }

    fn get(&mut self, key: K) -> &mut V {
        return self.map.entry(key.clone()).or_insert_with(|| (self.func)(key));
    }
}

pub fn reduce_vec<N: Num + PartialOrd + Copy + std::fmt::Debug>(mut vec: Vec<N>) -> Vec<N> {
    for i in (0..vec.len()).rev() {
        if vec[i] == N::zero() {
            vec.pop();
        } else {
            break;
        }
    }

    return vec;
}

pub fn new_rand_vec(size: usize) -> Vec<f64> {
    let mut rng = rand::rng();
    let mut vec: Vec<f64> = Vec::with_capacity(size);

    for _ in 0..size {
        vec.push(rng.random_range(-1.0..1.0));
    }

    return vec;
}

pub fn cartesian_products(n: usize) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    for i in 0..(1 << n) {
        let mut v = Vec::with_capacity(n);
        for j in (0..n).rev() {
            v.push(((i >> j) & 1) as u8);
        }
        result.push(v);
    }
    return result;
}

