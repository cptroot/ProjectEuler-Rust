use std::vec::{with_capacity, append};
use std::f64;

fn to_vec(mut n:int) -> ~[int] {
    let mut result: ~[int] = with_capacity(f64::floor(f64::log10(n as f64)) as uint + 1u);

    for _ in range(0, result.capacity()) {
        result.push(n % 10);
        n /= 10;
    }

    result
}

fn is_palindrome(n: int) -> bool {
    let nVec = to_vec(n);
    let mut nVecR = append(~[], nVec);
    nVecR.reverse();

    nVec == nVecR
}

fn main() {

    let mut max = 0;

    for i in range(100, 1000) {
        for j in range(i, 1000) {
            if i * j > max && is_palindrome(i * j) {
                max = i * j;
            }
        }
    }

    println!("{}", max);
}