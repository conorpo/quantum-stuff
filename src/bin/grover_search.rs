use std::collections::HashMap;
use std::env;
use quantum_stuff::dynamic::*;
use quantum_stuff::complex::*;

pub fn grover_search(n: usize, f: impl Fn(usize) -> usize, loops_hint: Option<usize>, trials: Option<usize>) -> usize {
    let trials: usize = trials.unwrap_or(10);
    let size = 1 << n;
    //had to put this 0.8 term is sqrt(2^n) seems to overestimate
    let loops = loops_hint.unwrap_or(((1usize << n) as f64 * 0.8).sqrt() as usize);

    //Gates
    let u_f = Gate::create_oracle(n, 1, f);
    let h_n = vec![Gate::hadamard();n].into_iter().reduce(|acc, cur| acc.tensor_product(&cur)).unwrap();
    let h = Gate::hadamard();
    let inversion_about_mean = {
        let entry = C64::new(1.0/(size as f64),0.0);
        let row = Vector::from_iter((0..size).map(|_| {
            entry
        }), Some(size));
        let a = Matrix::from_rows((0..size).map(|_| {
            row.clone()
        }), Some(size)).unwrap();

        Gate::try_from((a * C64::new(2.0,0.0) - &Matrix::eye(size)).unwrap()).unwrap()
    };


    let mut result_map = HashMap::<usize, usize>::new();
    for _ in 0..trials {
        let mut initial = State::from_qubits(vec![false; n].into_iter());

        initial.apply(&h_n);

        let mut output = State::from_qubit(true);
        output.apply(&h);
        let mut state = initial.tensor_product(output);
        for _ in 0..loops {
            // Phase Inversion / Function Evaluation
            state.apply(&u_f);
            state.apply_partial(0..n, &inversion_about_mean);
        }
        let res = state.measure_partial(0..n).0;

        result_map.entry(res).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut results: Vec<(usize, usize)> = result_map.into_iter().map(|p| (p.1, p.0)).collect();
    results.sort();

    //println!("{}%",results.last().unwrap().0 as f64 * 100.0 / trials as f64);
    
    results.last().unwrap().1
}


pub fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args.get(1).and_then(|n_string| n_string.parse().ok()).unwrap_or(3);
    let needle = args.get(2).and_then(|needle_string| needle_string.parse().ok()).unwrap_or(0);
    let trials_hint: Option<usize> = args.get(3).and_then(|s| Some(s.parse().ok())).unwrap_or(None);
    let loops_hint: Option<usize> = args.get(4).and_then(|s| Some(s.parse().ok())).unwrap_or(None);

    let f = |x: usize| -> usize {
        (x == needle) as usize
    };

    let res = grover_search(n, f, loops_hint, trials_hint);

    println!("The needle is: {}", res);
}

pub fn from_needle(needle: usize) -> impl Fn(usize) -> usize {
    move |x: usize| {
        (x == needle) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a_few() {
        for (n, needle) in [(5,11),(4,13),(6,2),(6,13),(5,17),(4,9),(3,3),(3,7),(2,1),(7,69)] {
            let res = grover_search(n, from_needle(needle), None, None);
            assert_eq!(res, needle);
        }
    }
}