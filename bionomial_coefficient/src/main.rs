const MAX_FACTORIAL: usize = 120;

fn main() {
    let mut factorials = [1u64; MAX_FACTORIAL + 1];

    for i in 2..=MAX_FACTORIAL {
        factorials[i] = factorials[i - 1] * i as u64;
    }

    let choose = |n: u64, k: u64| -> u64 {
        factorials[n as usize] / factorials[k as usize] / factorials[(n - k) as usize]
    };

    for i in 1..12 {
        println!("P(12,{}) = {}", i, choose(12, i));
    }
    for i in 10..60 {
        println!("C(60,{}) = {}", i, choose(60, i));
    }
}
