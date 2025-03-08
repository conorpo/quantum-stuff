#![feature(random)]

use std::{collections::HashSet, env, sync::LazyLock};
use std::random::random;

use quantum_stuff::dynamic::*;
use quantum_stuff::complex::*;

fn shor(N: u64) -> Result<(u64, u64), &'static str> {
    println!("Shor's algo for: {N}");
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

    // n = ceil(log(N))
    let n_bits = if N.is_power_of_two() {
        N.ilog2()
    } else {
        N.ilog2() + 1
    } as usize;

    let m_bits = 2 * n_bits;

    //Reused Gates
    let h_m = vec![Gate::hadamard();m_bits].into_iter().reduce(|acc, cur| acc.tensor_product(&cur)).unwrap();
    let inverse_qft = qft(1 << m_bits).inverse();

    let (a,r) = { 
        let mut a;
        let mut r: u64;
        loop {
            let rand: u64 = random();
            a = 2 + rand % (N - 2);
            //a = 5; //todo, testing with this for now, because chances are I will get an a that shares a factor, since I can really only test numbers under 16
            println!("a = {a}");
            f_cycle(a, N);

            let gcd_a_n = gcd(a, N);
            if gcd_a_n != 1 {
                return Ok((gcd_a_n, N / gcd_a_n));
            }

            let f = move |x: usize| {
                (mod_expo(a, x as u64, N)) as usize
            };

            let function_oracle = Gate::create_oracle_unchecked(m_bits, n_bits, f);

            let mut m_wire = State::from_qubits(vec![false; m_bits].into_iter());
            let n_wire = State::from_qubits(vec![false; n_bits].into_iter());
            m_wire.apply(&h_m);

            let mut mn_wire = m_wire.tensor_product(n_wire);
            mn_wire.apply(&function_oracle);

            let (n_measurement, mut m_wire) = mn_wire.measure_partial(m_bits..(m_bits+n_bits));

            m_wire.apply(&inverse_qft);

            let x = m_wire.measure();
            dbg!(x);
        
            //todo: account for r which don't evenly divide 2^m

            r = continued_fraction_expansion(x as u64, 1 << m_bits, N).unwrap().1;

            dbg!(r);

            if mod_expo(a, r, N) != 1 {
                println!("Why did this measure the wrong period?");
                continue;
            };
            if r % 2 == 1 { continue; }
            if mod_expo(a, r/2, N) == (N - 1) { continue; }
            break;
        }
        (a,r)
    };

    let gcd_a = gcd(a.pow(r as u32 / 2) + 1, N);
    let gcd_b = gcd(a.pow(r as u32 / 2) - 1, N);

    if gcd_a != 1 {
        Ok((gcd_a , N / gcd_a))
    } else if gcd_b != 1 {
        Ok((gcd_b, N / gcd_b))
    } else {
        panic!("How did we get here?");
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u64 = args.get(1).and_then(|n_string| n_string.parse().ok()).unwrap_or(15);

    let factors = shor(n).unwrap();
    
    println!("The prime factors of {n} are {} and {}.", factors.0, factors.1);
}


pub fn qft(n: usize) -> Gate {
    let scaling = C64::from_real(1.0 /(n as f64).sqrt());
    let first_root = C64::nth_root_of_unity(n);
    let mat = Matrix::from_rows((0..n).map(|k| {
        let temp_root = first_root.pow(k);
        let mut cur_root = C64::ONE;
        let mut row = Vector::zero(n);
        for j in 0..n {
            row.data[j] = cur_root * scaling;
            cur_root *= temp_root;
        }
        row
    }), Some(n)).unwrap();

    unsafe { Gate::from_matrix_unchecked(mat) }
}

pub fn continued_fraction_expansion(numerator: u64, denominator: u64, max_denominator: u64) -> Option<(u64,u64)>{
    let mut fractions = vec![];
    let mut a = numerator;
    let mut b = denominator;

    while b != 0 {
        fractions.push(a / b);
        (a, b) = (b, a % b);
    }

    // reconstruct the fraction from continued fraction terms
    let mut h1 = 1;
    let mut k1 = 0;
    let mut h = fractions[0];
    let mut k = 1;

    for &f in &fractions[1..] {
        let h2 = f * h + h1;
        let k2 = f * k + k1;
        if k2 > max_denominator {
            break;
        }
        (h1, k1) = (h, k);
        (h, k) = (h2, k2);
    }

    Some((h, k))
}

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

    println!("{:?}", list);
    println!("Period: {}", list.len());
    println!("");
}

pub fn mod_expo(a: u64, mut b: u64, m: u64) -> u64 {
    let mut mult = a;
    let mut res = 1;
    while b > 0 {
        if b & 1 == 1 {
            res *= mult;
            res %= m;
        }
        b >>= 1;
        mult *= mult;
        mult %= m;
    }
    res
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

const PRIME_MEMO_N: usize = 10000;
static IS_PRIME: LazyLock<[bool;PRIME_MEMO_N + 1]> = LazyLock::new(|| {
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
        return IS_PRIME[n as usize];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_is_unitary() {
        let f = move |x: usize| {
            mod_expo(5, x as u64, 6) as usize
        };

        let _ = Gate::create_oracle_unchecked(6, 3, f);

        let f = move |x: usize| {
            mod_expo(2, x as u64, 15) as usize
        };

        let _ = Gate::create_oracle_unchecked(8, 4, f);
    }

    #[test]
    fn test_mod_expo_periodicity() {
        for (a, n) in [(5,6), (12,25), (8,15),(15,247),(60,247)] {
            assert_eq!(gcd(a, n), 1);
            
            let n_bits = if n.is_power_of_two() {
                n.ilog2()
            } else {
                n.ilog2() + 1
            } as usize;
        
            let m_bits = 2 * n_bits;
        
            let mut results = vec![1]; 
            let mut i = 1usize;
            loop {
                let res = mod_expo(a, i as u64, n);
                i += 1;
                if res == 1 {break;}
                results.push(res);
            }
            let p = results.len();
            assert!(p < n as usize);
            println!("N: {n}, a: {a}, period: {p}");
            while i < 1 << m_bits {
                let res = mod_expo(a, i as u64, n);
                assert_eq!(results[i % p], res);
                i += 1;
            }
        }
    }

    #[test]
    fn test_qft() {
        let qft_gate = qft(4);
        println!("{}", qft_gate.get());
        //todo: make this actually test something
    }

    #[test]
    fn test_shors() {
        //f_cycle(5, 6);
        let res = shor(6).unwrap();
        assert!(res == (2,3) || res == (3,2));

        let res = shor(15).unwrap();
        assert!(res == (5,3) || res == (3,5));
        
        let res = shor(10).unwrap();
        assert!(res == (2,5) || res == (5,2));

        let res = shor(14).unwrap();
        assert!(res == (2,7) || res == (7,2));
    }
}