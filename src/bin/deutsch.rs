#![allow(non_snake_case)]

use quantum_stuff::dynamic::*;
use std::env;

#[derive(PartialEq, Debug)]
pub enum FType{
    Constant,
    Balanced
}

pub fn deutsch(f: impl Fn(usize) -> usize) -> FType {
    let U = Gate::create_oracle(1, 1, f);

    let mut input = State::from_qubit(false);
    let mut output = State::from_qubit(true);
    
    let H = Gate::hadamard();
    input.apply(&H);
    output.apply(&H);

    let mut state = input.tensor_product(output);    
    state.apply(&U);

    state.apply_partial(0..1, &H);
    
    match state.measure_partial(0..1).0 {
        0 => FType::Constant,
        1 => FType::Balanced,
        _ => panic!("How did we get here?")
    }
}


pub fn main() {
    let args: Vec<String> = env::args().collect();
    let f_0: usize = args.get(1).and_then(|n_string| n_string.parse().ok()).unwrap_or(0);
    let f_1: usize = args.get(2).and_then(|needle_string| needle_string.parse().ok()).unwrap_or(1);

    let f = |x: usize| {
        [f_0, f_1][x]
    };

    let res = deutsch(f);

    println!("The given function is {}", {
        match res {
            FType::Balanced => "BALANCED",
            FType::Constant => "CONSTANT",
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::{deutsch, FType};

    #[test]
    fn test_all() {
        assert_eq!(deutsch(|x: usize| {[0,0][x]}), FType::Constant);
        assert_eq!(deutsch(|x: usize| {[0,1][x]}), FType::Balanced);
        assert_eq!(deutsch(|x: usize| {[1,0][x]}), FType::Balanced);
        assert_eq!(deutsch(|x: usize| {[1,1][x]}), FType::Constant);
        
    }
}

