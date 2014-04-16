#[crate_id = "factor#0.1"];
#[crate_type="lib"];

use std::hashmap::HashSet;
use std::iter::range_step;
use std::vec;
use std::f64;
use std::num;



pub fn sieve(limit:uint) -> HashSet<uint> {
    let mut sieve : ~[bool] = vec::with_capacity(limit);
    let mut result : HashSet<uint> = HashSet::with_capacity(limit / 4);

    for _ in range_step(0u, limit - 1, 2) {
        sieve.push(false);
        sieve.push(true);
    }
    sieve[1] = false;
    sieve[2] = true;
    if (limit - 1) % 2 == 0 {sieve.push(false)}

    let sqrt_limit:uint = f64::ceil(f64::sqrt(limit as f64)) as uint;

    for i in range_step(3u, sqrt_limit, 2) {
        if !sieve[i] {continue;}
        result.insert(i);

        for n in range_step(3u, limit / i, 2) {
            sieve[i * n] = false;
        }
    }

    for i in range_step((sqrt_limit / 2) * 2 - 1, limit, 2) {
        if !sieve[i] {continue;}
        result.insert(i);
    }

    result
}

struct Factor {
    p: uint,
    n: uint
}

impl Clone for Factor {
    fn clone(&self) -> Factor {
        return Factor {p:self.p, n:self.n};
    }
    fn clone_from(&mut self, source: &Factor) {
        self.p = source.p;
        self.n = source.n;
    }
}

pub fn factor(input:uint, primes:HashSet<uint>) -> ~[Factor] {
     fn factor_helper(input:uint, mut n:uint, primes:HashSet<uint>) -> ~[Factor] {
        if input == 1 {
            let mut result: ~[Factor] = vec::with_capacity(0);
            return result;
        }

        while !primes.contains(&n) || input % n != 0 {
            n = n + 1;
        }

        return vec::append(factor_helper(input / n, n, primes), [Factor {p:n, n:1}]);
    }

    let mut result: ~[Factor] = factor_helper(input, 2, primes);
    result.reverse();

    result = factor_flatten(result);
    result.sort_by(|a, b| a.p.cmp(&(b.p)));

    result
}

fn factor_flatten(factors:&[Factor]) -> ~[Factor] {
    let mut result : ~[Factor] = vec::with_capacity(0);

    let mut current_factor: ~Factor = ~Factor {p:0, n:0};

    for factor in factors.iter() {
        if current_factor.p == factor.p {
            current_factor = ~Factor {p:factor.p, n:current_factor.n + factor.n};
        } else {
            result = vec::append(result, [*current_factor]);
            current_factor = ~(factor.clone());
        }
    }

    result
}

/*pub fn lcm(a:&[Factor], b:&[Factor]) {
    let result = ~[Factor] = with_capacity(num::max(a.length(), b.length()));

    let a_iter = a.iter().peekable();
    let b_iter = b.iter().peekable();

    while a_iter.peek().is_some() &&
}*/

/*pub fn number(input:&[Factor]) -> uint {
    let mut result = 1u;

    for factor in input.iter() {
        result *= num::pow(factor.p, factor.n);
    }

    result
}*/
