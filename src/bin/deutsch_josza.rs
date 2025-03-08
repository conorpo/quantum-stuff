#![allow(non_snake_case)]

use quantum_stuff::dynamic::*;

#[derive(PartialEq, Debug)]
pub enum FType{
    Constant,
    Balanced
}

// Assumes f is EITHER Constant or Balanced.
pub fn deutsch_josza(n: usize, f: impl Fn(usize) -> usize) -> FType {
    let U = Gate::create_oracle(n, 1, f);
    let H_N = vec![Gate::hadamard(); n].into_iter().reduce(|acc, cur| acc.tensor_product(&cur)).unwrap();

    let mut input = vec![State::from_qubit(false);n].into_iter().reduce(|acc, cur| acc.tensor_product(cur)).unwrap();
    let mut output = State::from_qubit(true);
    
    input.apply(&H_N);
    output.apply(&Gate::hadamard());

    let mut state = input.tensor_product(output);    
    state.apply(&U);


    state.apply_partial(0..n, &H_N);
    
    let res = state.measure_partial(0..n).0;
    match res {
        0 => FType::Constant,
        _ => FType::Balanced
    }
}

pub fn main() {
    println!("This shouldn't be a bin...");
}

#[cfg(test)]
mod tests {
    use crate::{deutsch_josza, FType};

    #[test]
    fn test_a_few() {
        let f = |x: usize| -> usize {
            x % 2
        };
        assert_eq!(deutsch_josza(2, f), FType::Balanced);
        assert_eq!(deutsch_josza(5, f), FType::Balanced);
        assert_eq!(deutsch_josza(7, f), FType::Balanced);

        let f = |_: usize| -> usize {
            1
        };
        assert_eq!(deutsch_josza(2, f), FType::Constant);
        assert_eq!(deutsch_josza(5, f), FType::Constant);
        assert_eq!(deutsch_josza(7, f), FType::Constant);

        let f = |_: usize| -> usize {
            0
        };
        assert_eq!(deutsch_josza(2, f), FType::Constant);
        assert_eq!(deutsch_josza(5, f), FType::Constant);
        assert_eq!(deutsch_josza(7, f), FType::Constant);
    }

    #[test]
    fn test_consistency() {
        let f = |x: usize| -> usize {
            x % 2
        };

        let count = (0..100).filter(|_| deutsch_josza(5, f) == FType::Balanced).count();
        assert_eq!(count, 100);

        let f = |_: usize| -> usize {
            1
        };

        let count = (0..100).filter(|_| deutsch_josza(5, f) == FType::Constant).count();
        assert_eq!(count, 100);
    }

}