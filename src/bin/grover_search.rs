use std::collections::HashMap;
use std::env;
use quantum_stuff::dynamic::*;
use quantum_stuff::complex::*;


pub fn main() {
    const TRIALS: usize = 50;
    let args: Vec<String> = env::args().collect();
    let n: usize = args.get(1).and_then(|n_string| n_string.parse().ok()).unwrap_or(3);
    let needle = args.get(2).and_then(|needle_string| needle_string.parse().ok()).unwrap_or(0);
    // Added this argument because sqrt(2^n) loops seems to overestimate, for n == 3..8
    let loop_change: i32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
    let loops = ((1i32 << n).isqrt() + loop_change) as usize;
    println!("Running {TRIALS} trials, repeating phase inversion {} times", loops);

    let size = 1 << n;
    assert!(needle < (1 << n));
    let f = |x: usize| -> usize {
        (x == needle) as usize
    };

    let U = Gate::create_oracle(size, 1, f);
    let H_N = vec![Gate::hadamard();n].into_iter().reduce(|acc, cur| acc.tensor_product(&cur)).unwrap();
    let H = Gate::hadamard();
    let inversion_about_mean = {
        let entry = C64::new(1.0/(size as f64),0.0);
        let row = Vector::from_iter((0..size).map(|_| {
            entry
        }), Some(size));
        let A = Matrix::from_rows((0..size).map(|_| {
            row.clone()
        }), Some(size)).unwrap();

        Gate::try_from((A * C64::new(2.0,0.0) - &Matrix::eye(size)).unwrap()).unwrap()
    };
    let mut result_map = HashMap::<usize, usize>::new();

    for t in 0..TRIALS {
        let mut initial = State::from_qubits(vec![false; n].into_iter());

        //phi_1

        initial.apply(&H_N);

        //phi_2

        let mut phase_oracle = State::from_qubit(true);
        phase_oracle.apply(&H);
        let mut state = initial.tensor_product(phase_oracle);
        for _ in 0..loops {
            // Phase Inversion / Function Evaluation
            state.apply(&U);
            state.apply_partial(0..n, &inversion_about_mean);
        }
        let res = state.measure_partial(0..n);

        result_map.entry(res).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut results: Vec<(usize, usize)> = result_map.into_iter().map(|p| (p.1, p.0)).collect();
    results.sort();
    
    // dbg!(&results);

    let res = results.last().unwrap();
    println!("The needle is: {}, found in {:2.1}% of trials", res.1, (res.0 as f64) * 100.0 / TRIALS as f64);
    assert_eq!(res.1, needle);
}