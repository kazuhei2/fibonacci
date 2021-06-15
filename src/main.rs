use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

fn fibonacci(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn main() {
    println!("fibonacci(8181): {}", fibonacci(8181));
}
