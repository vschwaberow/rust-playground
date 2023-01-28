use num_bigint::BigInt;
use num_traits::One;

fn p(mut n: u32, p: u32) -> BigInt {
    let mut r = BigInt::one();
    let k = n - p;
    while n > k {
        r *= n;
        n -= 1;
    }
    return r;
}

fn c(n: u32, mut k: u32) -> BigInt {
    let mut r = p(n, k);
    while k > 0 {
        r /= k;
        k -= 1;
    }
    return r;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 1..12 {
        println!("P(12,{}) = {}", i, p(12, i));
    }
    for i in 10..60 {
        println!("C(60,{}) = {}", i, c(60, i));
    }
    Ok(())
}
