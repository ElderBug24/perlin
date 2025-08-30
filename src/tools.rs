use std::collections::HashMap;
use std::hash::Hash;

use rand::Rng;
use num::Num;


#[derive(Debug)]
pub struct Cache<K: Eq + Hash + Clone, V> {
    map: HashMap<K, V>,
    func: fn(K) -> V
}

impl<K: Eq + Hash + Clone, V> Cache<K, V> {
    pub fn new(func: fn(K) -> V) -> Self {
        return Self {
            map: HashMap::new(),
            func: func
        };
    }

    pub fn with_capacity(func: fn(K) -> V, capacity: usize) -> Self {
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
    if n >= usize::BITS as usize {
        panic!("n shouldn't be greater than {} (architecture dependent)", usize::BITS);
    }
    let mut result = Vec::new();
    for i in 0..(1usize << n) {
        let mut v = Vec::with_capacity(n);
        for j in (0..n).rev() {
            v.push(((i >> j) & 1) as u8);
        }
        result.push(v);
    }
    return result;
}

