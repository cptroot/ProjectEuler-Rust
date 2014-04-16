extern mod factor;
use std::iter::range_step;

fn main() {
    let primes = factor::sieve(1000000);

    let mut num = 600851475143i64;
    let mut max = 0;

    for i in range_step(3u, 1000000, 2) {
        if !primes.contains(&i) {continue;}

        while num % (i as i64) == 0 {
            max = i;
            num /= i as i64;
        }
    }

    println!("{}", max);
}
