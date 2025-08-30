use std::collections::HashMap;
use std::hash::Hash;

use rand::Rng;
use num::Num;


pub struct PerlinNoiseMap {
    vector_map: HashMap<Vec<isize>, Vec<f64>>
}

impl PerlinNoiseMap {
    pub fn new() -> Self {
        return Self{
            vector_map: HashMap::new()
        };
    }

    pub fn with_capacity(capacity: usize) -> Self {
        return Self {
            vector_map: HashMap::with_capacity(capacity)
        };
    }

    pub fn show(&self) {
        println!("{:#?}", self.vector_map);
    }

    pub fn get_vector(&mut self, pos: &Vec<isize>) -> &Vec<f64> {
        let rpos = reduce_vec::<isize>(pos.clone());
        return {
            let mut rng = rand::rng();
            let v = self.vector_map.entry(rpos).or_insert_with(|| new_rand_vec(pos.len()));

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

        todo!();
    }
}

pub struct Cache<K: Eq + Hash + Clone, V, F> where F: Fn(K) -> V {
    map: HashMap<K, V>,
    func: F
}

impl<K: Eq + Hash + Clone, V, F> Cache<K, V, F> where F: Fn(K) -> V {
    pub fn new(func: F) -> Self {
        return Self{
            map: HashMap::new(),
            func: func
        };
    }

    pub fn with_capacity(func: F, capacity: usize) -> Self {
        return Self {
            map: HashMap::with_capacity(capacity),
            func: func
        };
    }

    pub fn get(&mut self, key: K) -> &mut V {
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

