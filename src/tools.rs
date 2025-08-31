use std::collections::HashMap;
use std::hash::Hash;

use rand::Rng;
use num::Num;


#[derive(Debug)]
pub struct Cache<K: Eq + Hash + Clone, V, A> {
    cache: HashMap<K, V>,
    func: fn(K, A) -> V
}

impl<K: Eq + Hash + Clone, V, A> Cache<K, V, A> {
    pub fn new(func: fn(K, A) -> V) -> Self {
        return Self {
            cache: HashMap::new(),
            func: func
        };
    }

    pub fn with_capacity(func: fn(K, A) -> V, capacity: usize) -> Self {
        return Self {
            cache: HashMap::with_capacity(capacity),
            func: func
        };
    }

    pub fn get(&mut self, key: K, args: A) -> &mut V {
        return self.cache.entry(key.clone()).or_insert_with(|| (self.func)(key, args));
    }
}

#[derive(Debug)]
pub struct CacheFixedCapacity<K: Eq + Hash + Clone, V> {
    capacity: usize,
    cache: HashMap<K, V>,
    func: fn(K) -> V
}

impl<K: Eq + Hash + Clone, V> CacheFixedCapacity<K, V> {
    pub fn new(func: fn(K) -> V, capacity: usize) -> Self {
        return Self {
            capacity: capacity,
            cache: HashMap::with_capacity(capacity),
            func: func
        };
    }

    pub fn get(&mut self, key: K) -> &mut V {
        if self.cache.contains_key(&key) {
            return self.cache.get_mut(&key).unwrap();
        } else {
            if self.cache.len() == self.capacity {
                self.cache.remove(&self.cache.keys().next().unwrap().clone());
            }

            return self.cache.entry(key.clone()).or_insert_with(|| (self.func)(key));
        }
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

pub fn new_rand_vec(capacity: usize) -> Vec<f64> {
    let mut rng = rand::rng();
    let mut vec: Vec<f64> = Vec::with_capacity(capacity);

    for _ in 0..capacity {
        vec.push(rng.random_range(-1.0..1.0));
    }

    return vec;
}

pub fn cartesian_products(n: usize) -> Vec<Vec<u8>> {
    assert!(n < usize::BITS as usize);
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

pub fn flat_nd_lerp(pos: &Vec<f64>, corners: &Vec<Vec<u8>>, values: &Vec<f64>) -> f64 {
    let one_minus_pos: Vec<f64> = pos.iter().map(|&x| 1.0 - x).collect();
    let weights: Vec<f64> = corners
        .iter()
        .map(|corner| {
            corner
                .iter()
                .enumerate()
                .map(|(i, &c)| if c == 1 { pos[i] } else { one_minus_pos[i] })
                .product::<f64>()
        })
        .collect();

    weights
        .iter()
        .zip(values.iter())
        .map(|(w, v)| w * v)
        .sum()
}

