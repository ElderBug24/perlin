use std::collections::HashMap;
use std::hash::Hash;

use rand::Rng;
use num_traits::{Num, FromPrimitive};


#[derive(Debug)]
pub struct VectorCache<K, V, A> {
    cache: HashMap<Vec<K>, V>,
    func: fn(Vec<K>, A) -> V
}

impl<K: Eq + Hash + Clone, V, A> VectorCache<K, V, A> {
    pub fn new(func: fn(Vec<K>, A) -> V) -> Self {
        return Self {
            cache: HashMap::new(),
            func: func
        };
    }

    pub fn with_capacity(func: fn(Vec<K>, A) -> V, capacity: usize) -> Self {
        return Self {
            cache: HashMap::with_capacity(capacity),
            func: func
        };
    }

    pub fn get(&mut self, key: &[K], args: A) -> &mut V {
        if self.cache.contains_key(key) {
            return self.cache.get_mut(key).unwrap();
        }

        self.cache.insert(key.to_vec(), (self.func)(key.to_vec(), args));

        return self.cache.get_mut(key).unwrap();
    }

    pub fn remove(&mut self, key: &[K]) -> Option<V> {
        return self.cache.remove(key);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

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
        if self.cache.contains_key(&key) {
            return self.cache.get_mut(&key).unwrap();
        }

        self.cache.insert(key.clone(), (self.func)(key.clone(), args));

        return self.cache.get_mut(&key).unwrap();
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        return self.cache.remove(&key);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
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

pub fn reduce_vec<'a, N: Num + PartialOrd + Copy + std::fmt::Debug>(vec: &'a Vec<N>) -> &'a [N] {
    for i in (0..vec.len()).rev() {
        if vec[i] != N::zero() {
            return &vec[0..i+1];
        }
    }

    return &vec[0..1];
}

pub fn new_rand_vec(capacity: usize) -> Vec<f64> {
    let mut rng = rand::rng();
    let mut vec: Vec<f64> = Vec::with_capacity(capacity);

    for _ in 0..capacity {
        vec.push(rng.random_range(-1.0..1.0));
    }

    return vec;
}

pub fn cartesian_products<T: Num + FromPrimitive>(n: usize) -> Vec<Vec<T>> {
    // assert!(n < usize::BITS as usize);
    let result: Vec<Vec<T>> = (0..(1usize << n))
        .map(|i| {
            (0..n)
                .rev()
                .map(|j| T::from_usize((i >> j) & 1).unwrap())
                .collect()})
        .collect();

    return result;
}

pub fn flat_nd_lerp(pos: &Vec<f64>, corners: &Vec<Vec<f64>>, values: &Vec<f64>) -> f64 {
    return corners
        .iter()
        .zip(values)
        .map(|(corner, v)| {
            corner
                .iter()
                .enumerate()
                .map(|(i, &c)| (1.0 - c - pos[i]).abs())
                .product::<f64>() * v
        })
        .sum();
}

