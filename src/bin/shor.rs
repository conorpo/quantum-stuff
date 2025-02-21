#![feature(random)]

use std::{collections::HashSet, env, sync::LazyLock};
use std::random::random;

pub fn f_cycle(a: u64, N: u64) {
    let mut set = HashSet::new();
    let mut list = Vec::new();
    let mut cur = 1;
    loop {
        set.insert(cur);
        list.push(cur);
        cur *= a;
        cur %= N;
        if set.contains(&cur) { break; }
    }

    println!("a = {a}");
    println!("{:?}", list);
    println!("Period: {}", list.len());
    println!("");
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

const PRIME_MEMO_N: usize = 10000;
static is_prime_memo: LazyLock<[bool;PRIME_MEMO_N + 1]> = LazyLock::new(|| {
    let mut is_prime = [true; PRIME_MEMO_N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for n in 2..PRIME_MEMO_N {
        if !is_prime[n] {continue;}
        let mut cur = n * 2;
        while cur <= PRIME_MEMO_N {
            is_prime[cur] = false;
            cur += n;
        }
    }
    is_prime
});


pub fn phi(n: u64) ->  u64 {
    let mut prime_divisors = Vec::new();
    let mut cur_n = n;
    let mut d = 2;
    while d < cur_n {
        if cur_n % d == 0 {
            prime_divisors.push(d);
            cur_n /= d;
            while cur_n % d == 0 {
                cur_n /= d;
            }
        }
        d += 1;
    }

    let mut n = n;
    for prime_divisor in prime_divisors {
        n /= prime_divisor;
        n *= prime_divisor - 1;
    }
    n
}

pub fn is_prime(n: u64) -> bool {
    if n as usize <= PRIME_MEMO_N {
        return is_prime_memo[n as usize];
    }

    let sqrt_n = n.isqrt();

    //Perfect power test
    for a in 2..=sqrt_n {
        let mut cur = a;
        while cur <= n {
            if cur == n { return false; }
            cur *= a;
        }
    }

    let mut r = 2;
    loop {
        if gcd(r, n) != 1 { return false; }
        let order = {
            let mut i = 1;
            let mut cur = n % r;
            while cur != 1 {
                cur *= n;
                cur %= r;
                i += 1;
            }
            i
        };
        if order as f64 > (n as f64).log2().powi(2) { break; }
        r += 1;
    }

    if (2..=r.min(n-1)).any(|d| n % d == 0) { return false; }

    if n <= r { return true; }

    let upper = ((phi(r) as f64).sqrt() * (n as f64).log2()) as u64;
    for a in 1..=upper {
        //tricky quotient ring multiplication
    }
    todo!()

}

fn shor(N: u64) -> Result<(u64, u64), &'static str> {
    if is_prime(N) { return Err("N is prime."); }

    //Power of prime test
    let sqrt_n = N.isqrt();
    for a in 2..=sqrt_n {
        if !is_prime(a) {continue;}
        let mut cur = a * a;
        while cur <= N {
            if cur == N { return Err("N is a power of a prime"); }
            cur *= a;
        }
    }

    loop {
        let rand: u64 = random();
        let a = 2 + rand % (N - 2);
        let a = 2; //todo, testing with this for now, because chances are I will get an a that shared a factor

        let gcd_a_n = gcd(a, N);
        if gcd_a_n != 1 {
            return Ok((gcd_a_n, N / gcd_a_n));
        }



    }


    let n_bits = if N.is_power_of_two() {
        N.ilog2()
    } else {
        N.ilog2() + 1
    };

    let m_bits = 2 * n_bits;

    let a = {
        let mut a = 2;
        //while gcd
    };

    todo!();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let N: u64 = args.get(1).and_then(|n_string| n_string.parse().ok()).unwrap_or(15);

   
    
    f_cycle(2, N);
}