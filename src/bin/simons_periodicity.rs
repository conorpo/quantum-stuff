#![allow(non_snake_case)]

use quantum_stuff::dynamic::*;
use std::collections::HashSet;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args.get(1).and_then(|n_string| n_string.parse().ok()).unwrap_or(1);
    let c: usize  = args.get(2).and_then(|needle_string| needle_string.parse().ok()).unwrap_or(1);

    let f = from_c(n, c);

    let res = simons_periodicity(n, f);
    println!("c is {res}");
}

fn from_c(n: usize, c: usize) -> impl Fn(usize) -> usize {
    let mut output_vec = vec![0;2usize.pow(n as u32)];
    let mut pairs = 0;
    for i in 0..2usize.pow(n as u32) {
        if (i ^ c) < i {
            output_vec[i] = output_vec[i ^ c];
        } else {
            output_vec[i] = pairs;
            pairs += 1;
        }
    }

    move |x: usize| -> usize {
        output_vec[x]
    }
}

pub fn simons_periodicity(n: usize, f: impl Fn(usize) -> usize) -> usize {
    //Garunteed to be unitary
    let U = Gate::create_oracle_unchecked(n, n, f);

    let H_N = vec![Gate::hadamard();n].into_iter().reduce(|acc, cur| acc.tensor_product(&cur)).unwrap();
    let H_N_I_N = H_N.tensor_product(&Gate::identity(2usize.pow(n as u32)));

    //Apparently this only works consistently if the set of answers are linearly independent

    let mut set = HashSet::new();
    let mut vals = Vec::new();
    while vals.len() < (n - 1) {
        let mut input = State::from_qubits(vec![false; n].into_iter());
        
        // phi_0

        input.apply(&H_N);
        let output = State::from_qubits(vec![false; n].into_iter());
        let mut state = input.tensor_product(output);

        state.apply(&U);

        state.apply(&H_N_I_N);

        let res = state.measure_partial(0..n);

        if !set.contains(&res) && independent(n, [vals.clone(),vec![res]].concat()) {
            set.insert(res);
            vals.push(res);
        }
    }
    
    solve_xor_homo_system(n, vals)
}

pub fn independent(n: usize, mut vals: Vec<usize>) -> bool {
    let mut rank = 0;

    //Row reduction
    for col in 0..n {
        let col_bit = 1 << (n - col - 1);
        if let Some(idx) = vals.iter().skip(rank).position(|probe| probe & col_bit != 0) {
            
            let idx = idx + rank;
            vals.swap(rank, idx);

            let pivot_row = vals[rank];
            rank += 1;

            for val in vals.iter_mut().skip(rank) {
                if *val & col_bit != 0 { *val ^= pivot_row };
            }
        }
    }

    rank == vals.len()
}

pub fn solve_xor_homo_system(n: usize, mut vals: Vec<usize>) -> usize{
    // assert_eq!(vals.len(), n);
    let mut pivot_cols = vec![false;n];
    let mut rank = 0;

    //Row reduction
    for col in 0..n {
        let col_bit = 1 << (n - col - 1);
        if let Some(idx) = vals.iter().skip(rank).position(|probe| probe & col_bit != 0) {
            
            let idx = idx + rank;
            vals.swap(rank, idx);

            let pivot_row = vals[rank];
            pivot_cols[col] = true;
            rank += 1;

            for val in vals.iter_mut().skip(rank) {
                if *val & col_bit != 0 { *val ^= pivot_row };
            }
        }
    }
    
    let mut solution = 0usize;
    let first_free = pivot_cols.iter().position(|probe| !probe).unwrap();
    solution |= 1 << (n - first_free - 1);
    for (_, row) in vals.iter().enumerate().rev() {
        if *row == 0 {continue;}
        let pivot_col = n - row.ilog2()  as usize - 1;
        let mut xor = 0;
        for c in (pivot_col+1)..n {
            xor ^= ((solution & (1 << (n - c - 1)) & row) != 0) as usize;
        }
        solution |= xor << (n - pivot_col -1);
    }

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_few() {
        for (n, c) in [(3,5),(5,11),(5,13),(5,31), (4,13), (3,7), (4,9)] {
            let f = from_c(n,c);
            let c_prime = simons_periodicity(n, f);
            assert_eq!(c, c_prime);
        }
    }

    #[test]
    fn test_consistency() {
        let c = 13;
        let f = from_c(4, c);
        for _ in 0..100 {
            assert_eq!(simons_periodicity(4, &f), c);
        };
    }
}