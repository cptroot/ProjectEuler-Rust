extern mod factor;

use std::iter::range;

fn main() {
    let primes = factor::sieve(20);

    let mut total = factor::factor(1);

    for i in range(1, 20) {
        total = factor::lcm(total, factor::factor(i));
    }

    println!("{}", factor::number(total));
}
