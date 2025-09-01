use perlin::*;

use rand::Rng;


fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    return sum / (data.len() as f64);
}

fn median(data: &mut [f64]) -> f64 {
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        (data[mid - 1] + data[mid]) / 2.0
    } else {
        data[mid]
    }
}

fn std_dev(data: &[f64]) -> f64 {
    let m = mean(data);
    let variance = data.iter()
        .map(|value| {
            let diff = *value - m;
            diff * diff
        })
        .sum::<f64>() / (data.len() as f64);
    variance.sqrt()
}

const N: u32 = 3000000;
// const N: u32 = 50000000;

const N_: u32 = 35;
// const N_: u32 = 1;

fn test_nmap(n: u32) {
    let mut rng = rand::rng();
    let mut nm = NoiseMap::new(default_layers(n, 0.5));

    let mut vec: Vec<f64> = (0..N)
        .map(|_| nm.get(&vec![rng.random::<f64>() * N as f64 / 10.0, rng.random::<f64>() * N as f64 / 10.0]))
        .collect();

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // let mean = mean(&vec);
    // let std_dev = std_dev(&vec);
    // let median = median(&mut vec);

    let min = vec
        .iter()
        .next()
        .unwrap();

    let max = vec
        .iter()
        .last()
        .unwrap();

    println!();
    println!("n: {}", n);
    println!("{}", vec![max, &-min].into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
    // println!("Len: {}", N);
    // println!("Min: {}", min);
    // println!("Max: {}", max);
    // println!("Mean: {}", mean);
    // println!("Median: {}", median);
    // println!("Std. dev.: {}", std_dev);
}

fn main() {
    for i in 1..=N_ {
        test_nmap(i);
    }
}
