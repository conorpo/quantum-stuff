use super::vector::*;
use crate::complex::*;
use super::operator::*;
use std::{ops::Range, random::random};

// At this point keep state invariants (normalized, etc..)
pub struct State(Vector<C64>);


impl State {
    pub fn new(vector: Vector<C64>) -> Result<Self, ()> {
        if (vector.norm() - 1.0).abs() > f64::EPSILON * 5.0 {
            return Err(());
        }

        Ok(Self(vector))
    }

    pub fn qubit(enabled: bool) -> Self {
        Self(
            Vector::from(if enabled {
                &[C64::ZERO, C64::ONE][..]
            } else {
                &[C64::ONE, C64::ZERO]
            })
        )
    }

    pub fn tensor_product(self, rhs: Self) -> Self {
        Self(
            Vector::from_iter(self.0.iter().map(|&entry| rhs.0.iter().map(move |r_entry| entry * *r_entry)).flatten(), Some(self.0.dim() * rhs.0.dim()))
        )    
    }

    pub fn apply(&mut self, op: &Operator) {
        if op.get().dim().0 != self.0.dim() {
            panic!("Provided operator dimension does not match state dimension");
        }
        self.0 = (op.get() * &self.0).unwrap();
    }

    pub fn apply_partial(&mut self, interval: Range<usize> ,op: &Operator) {
        assert_eq!(2usize.pow(interval.len() as u32), op.dim());

        todo!();
    }

    pub fn qubits(&self) -> usize {
        self.0.dim().ilog2() as usize
    }

    pub fn measure(&mut self) -> usize {
        let rand: u32 = random::<u32>().min(u32::MAX - 1);
        let sample = (rand as f64) / (u32::MAX as f64);
        let mut probs = self.0.iter().map(|entry| entry.modulus_squared());
        let mut sum = 0.0;

        let mut measured = None;
        for (i, prob) in probs.enumerate() {
            sum += prob;
            if sample < sum {
                measured = Some(i);
            }
        };

        if let Some(measured) = measured {
            self.0.data.iter_mut().for_each(|entry| *entry = C64::ZERO);
            self.0.data[measured] = C64::ONE;
            measured
        } else {
            panic!("How did we get here");
        }
    }

    pub fn measure_partial(&mut self, interval: Range<usize>) -> usize {
        let res = self.measure();
        let q = self.qubits();
        (res / (2 << (q - interval.end))) % (2 << (q - interval.start))
    }
}