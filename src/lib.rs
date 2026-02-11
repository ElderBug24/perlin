pub mod tools;

pub use crate::tools::*;

use std::cmp::max;

use rand::Rng;
use num_traits::{Float, FromPrimitive};


pub fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

pub fn default_layers(depth: u32, falloff: f64) -> Vec<[f64; 2]> {
    return (0..depth)
        .map(|n| [(1.0/falloff).powi(n as i32), falloff.powi(n as i32)])
        .collect();
}

const PERLIN_NOISE_MAP_VECTOR_MAP_FUNC: fn(Vec<isize>, usize) -> Vec<f64> = |_vec: Vec<isize>, len: usize| { new_rand_vec(len) };
const PERLIN_NOISE_MAP_CARTESIAN_PRODUCTS_FUNC: fn(usize, ()) -> Vec<Vec<f64>> = |n: usize, _| { cartesian_products::<f64>(n) };

pub struct PerlinNoiseMap {
    scale: f64,
    vector_map: VectorCache<isize, Vec<f64>, usize>,
    cartesian_products_cache: Cache<usize, Vec<Vec<f64>>, ()>,

    cpos: Vec<isize>
}

impl PerlinNoiseMap {
    pub fn new(scale: f64) -> Self {
        return Self {
            scale: scale,
            vector_map: VectorCache::new(PERLIN_NOISE_MAP_VECTOR_MAP_FUNC),
            cartesian_products_cache: Cache::new(PERLIN_NOISE_MAP_CARTESIAN_PRODUCTS_FUNC),

            cpos: Vec::new()
        };
    }

    pub fn with_capacity(scale: f64, capacity: usize) -> Self {
        return Self {
            scale: scale,
            vector_map: VectorCache::with_capacity(PERLIN_NOISE_MAP_VECTOR_MAP_FUNC, capacity),
            cartesian_products_cache: Cache::new(PERLIN_NOISE_MAP_CARTESIAN_PRODUCTS_FUNC),

            cpos: Vec::new()
        };
    }

    pub fn show(&self) {
        println!("{:#?}", self.vector_map);
        println!("{:#?}", self.cartesian_products_cache);
    }

    pub fn get_vector(&mut self, pos: &Vec<isize>) -> &[f64] {
        let mut rng = rand::rng();
        let reduced_pos = reduce_vec::<isize>(&pos);
        let v = self.vector_map.get(reduced_pos, pos.len());

        for _ in 0..(max(pos.len() - v.len(), 0)) {
            v.push(rng.random_range(-1.0..1.0));
        }

        return &v[0..pos.len()];
    }

    pub fn get_vector_map(&mut self) -> &VectorCache<isize, Vec<f64>, usize> {
        return &self.vector_map;
    }

    pub fn clear_vector_map(&mut self) {
        self.vector_map.clear();
    }

    pub fn remove_from_vector_map(&mut self, pos: &Vec<isize>) -> Option<Vec<f64>> {
        return self.vector_map.remove(pos);
    }

    pub fn get(&mut self, pos: &Vec<f64>) -> f64 {
        let corners = self.cartesian_products_cache.get(pos.len(), ()).clone();

        self.cpos.clear();
        self.cpos.reserve(pos.len());

        let mut rpos: Vec<f64> = Vec::with_capacity(pos.len());
        let mut fpos: Vec<f64> = Vec::with_capacity(pos.len());
        let mut vpos: Vec<isize> = Vec::with_capacity(pos.len());

        pos
            .iter()
            .for_each(|&n| {
                let n = n * self.scale;
                let c = n.floor();
                let r = n - c;

                self.cpos.push(c as isize);
                rpos.push(r);
                fpos.push(fade(r));
            });

        let result = corners
            .iter()
            .map(|c| {
                vpos.clear();
                c
                    .iter()
                    .zip(&self.cpos)
                    .for_each(|(&c, &cp)| vpos.push(c as isize + cp));
                let vector = self.get_vector(&vpos);

                let mut product = 1.0;
                c
                    .iter()
                    .enumerate()
                    .zip(&rpos)
                    .zip(vector)
                    .map(|(((i, &c), &rp), v)| {
                        product *= (1.0 - c - fpos[i]);
                        (c - rp) * v
                    })
                    .sum::<f64>() * product.abs()
            })
            .sum();

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
        let perlin_noise_maps: Vec<PerlinNoiseMap> = layers
            .iter()
            .map(|layer| PerlinNoiseMap::new(layer[0]))
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
        let perlin_noise_maps: Vec<PerlinNoiseMap> = layers
            .iter()
            .map(|layer| PerlinNoiseMap::with_capacity(layer[0], capacity))
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
            .map(|(noise_map, layer)| noise_map.get(&pos) * layer[1])
            .sum::<f64>();

        return result / self.total_coeff;
    }

    pub fn show(&self) {
        for perlin_noise_map in &self.perlin_noise_maps {
            perlin_noise_map.show();
        }
    }
}

fn _render_memory_efficient(_layers: Vec<[f64; 2]>, _ranges: Vec<[f64; 3]>) -> Vec<Vec<f64>> {
    // let mut noise_map = NoiseMap::new(layers);

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

// vectors: widt+1 * height+1 * 2
// output: width*resx * height*resy
pub fn render_arr_2d<T: Float+FromPrimitive+std::fmt::Display+std::fmt::Debug+std::ops::MulAssign+std::iter::Sum>(vectors: &[T], output: &mut [T], (width, height): (usize, usize), (resx, resy): (usize, usize), coeff: T) {
    let zero = T::zero();
    let one = T::one();
    // let corners = [(zero, zero), (one, zero), (zero, one), (one, one)];
    let corners = [[0, 0], [1, 0], [0, 1], [1, 1]];

    let fade: fn(T) -> T = |t| t * t * t * (t * (t * T::from_u8(6).unwrap() - T::from_u8(15).unwrap()) + T::from_u8(10).unwrap());

    for y in 0..(height * resy) {
        for x in 0..(width * resx) {
            let cx = x / resx;
            let cy = y / resy;
            // let rx = T::from_usize(x % resx).unwrap() / T::from_usize(resx).unwrap();
            // let ry = T::from_usize(y % resy).unwrap() / T::from_usize(resy).unwrap();

            let rx = T::from_usize(x-cx*resx).unwrap() / T::from_usize(resx).unwrap();
            let ry = T::from_usize(y-cy*resy).unwrap() / T::from_usize(resy).unwrap();

            // println!("(x: {:.2}, y: {:.2}): cx: {:.2}, cy: {:.2}, rx: {:.2}, ry: {:.2}", x, y, cx, cy, rx, ry);
            // let temp = [(vectors[dbg!(cx+cy*(width+1))], [0, 0]), (vectors[dbg!(cx+1+cy*(width+1))], [1, 0]), (vectors[dbg!(cx+(cy+1)*(width+1))], [0, 1]), (vectors[dbg!(cx+1+(cy+1)*(width+1))], [1, 1])];
            // let temp = [
            //     ((vectors[(cx+cy*(width+1))*2], vectors[(cx+cy*(width+1))*2+1]), (zero, zero)),
            //     ((vectors[(cx+1+cy*(width+1))*2], vectors[(cx+1+cy*(width+1))*2+1]), (one, zero)),
            //     ((vectors[(cx+(cy+1)*(width+1))*2], vectors[(cx+(cy+1)*(width+1))*2+1]), (zero, one)),
            //     ((vectors[(cx+1+(cy+1)*(width+1))*2], vectors[(cx+1+(cy+1)*(width+1))*2+1]), (one, one))
            // ];
            //
            // let mut dots = [zero; 4];
            // for i in 0..4 {
            //     let ((vx, vy), (dx, dy)) = temp[i];
            //
            //     dots[i] = vx * (dx - rx) + vy * (dy - ry);
            // }
            //
            // let rx = fade(rx);
            // let ry = fade(ry);
            //
            // let br = (one - rx) * dots[0] + rx * dots[1];
            // let tr = (one - rx) * dots[2] + rx * dots[3];
            // let r = (one - ry) * br + ry * tr;

            let cpos = [cx, cy];
            let rpos = [rx, ry];
            let fpos = [fade(rx), fade(ry)];

            let r = corners
                .iter()
                .map(|c| {
                    let vx = c[0] + cx; // coords
                    let vy = c[1] + cy;
                    let [vx, vy] = [vectors[(vx+vy*(width+1))*2], vectors[(vx+vy*(width+1))*2+1]];

                    let cx = T::from_usize(c[0]).unwrap();
                    let cy = T::from_usize(c[1]).unwrap();
                     ((cx - rx) * vx
                    + (cy - ry) * vy)
                    * ((one - cx - fade(rx))
                    *  (one - cy - fade(ry))).abs()
                })
                .sum::<T>();

            output[x+y*width*resx] = r * coeff;
        }
    }
}

