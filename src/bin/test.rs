use perlin::*;
use crate::tools::*;


fn main() {
    let mut c = CacheFixedSize::new(3, |k| 2*k);
    println!("{:?}", c);

    c.get(1);
    println!("{:?}", c);
    c.get(2);
    println!("{:?}", c);
    c.get(3);
    println!("{:?}", c);
    c.get(4);
    println!("{:?}", c);
    c.get(5);
    println!("{:?}", c);
    c.get(6);
    println!("{:?}", c);
}

