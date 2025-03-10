use super::vector::*;
use crate::complex::*;
use super::gate::*;
use std::{ops::Range, random::random};

// At this point keep state invariants (normalized, etc..)
#[derive(Clone)]
pub struct State(Vector<C64>);


impl State {
    pub fn get(&self) -> &Vector<C64> {
        &self.0
    }

    pub fn from_qubit(enabled: bool) -> Self {
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

    pub fn apply(&mut self, op: &Gate) {
        if op.get().dim().0 != self.0.dim() {
            panic!("Provided operator dimension does not match state dimension");
        }
        self.0 = (op.get() * &self.0).unwrap();
    }


    pub fn apply_partial(&mut self, interval: Range<usize> ,op: &Gate) {
        assert_eq!(2usize.pow(interval.len() as u32), op.dim());
        
        let left_size = 2_usize.pow(interval.start as u32);
        let right_size = self.0.dim() / 2_usize.pow(interval.end as u32);

        let left_id = Gate::identity(left_size);
        let right_id = Gate::identity(right_size);

        let full_op = left_id.tensor_product(op).tensor_product(&right_id);

        self.apply(&full_op)
    }

    pub fn num_qubits(&self) -> usize {
        self.0.dim().ilog2() as usize
    }

    pub fn from_qubits(qubits: impl Iterator<Item = bool>) -> Self {
        let mut state = State::try_from(Vector::from(&[C64::ONE][..])).unwrap();
        for qubit in qubits {
            state = state.tensor_product(State::from_qubit(qubit))
        }
        state
    }

    pub fn measure(&mut self) -> usize {
        let mut prob_prefix_sum = Vec::with_capacity(self.0.dim());
        let mut prob = 0.0;
        for i in 0..(1 << self.num_qubits()) {
            prob += self.0.get(i).modulus_squared();
            prob_prefix_sum.push(prob);
        }

        let mut measured = (1 << self.num_qubits());
        while measured == 1 << self.num_qubits() {
            let rand: u64 = random::<u64>().min(u64::MAX - 1);
            let sample = (rand as f64) / (u64::MAX as f64);
    
            measured = prob_prefix_sum.binary_search_by(|probe| {
                probe.partial_cmp(&sample).unwrap().then(std::cmp::Ordering::Greater)
            }).unwrap_err();
        }

        
        self.0.data.iter_mut().for_each(|entry| *entry = C64::ZERO);
        self.0.data[measured] = C64::ONE;
        measured
    }

    pub fn measure_partial(self, interval: Range<usize>) -> (usize, Self){
        let q = self.num_qubits();
        
        let mut prob_prefix_sum = Vec::new();

        let mut prob = 0.0;
        for m in 0..(1 << interval.len()) {
            for l in 0..(1 << interval.start) {
                for r in 0..(1 << (q - interval.end)) {
                    let k = (l << (q - interval.start)) + (m << (q - interval.end)) + r;
                    prob += self.0.get(k).modulus_squared();
                }
            }
            prob_prefix_sum.push(prob);
        }

        let mut measured = (1 << interval.len());
        while measured == (1 << interval.len()){
            let random_u64 = random::<u64>().min(u64::MAX - 1);
            let random_sample = (random_u64 as f64) / (u64::MAX as f64);
    
            measured = prob_prefix_sum.binary_search_by(|probe| {
                probe.partial_cmp(&random_sample).unwrap().then(std::cmp::Ordering::Greater)
            }).unwrap_err();
        }

        let mut new_state_vector = Vector::<C64>::zero(1 << (q - interval.len()));
        for l in 0..(1 << interval.start) {
            for r in 0..(1 << (q - interval.end)) {
                let k = (l << (q - interval.start)) + (measured << (q - interval.end)) + r;
                new_state_vector.data[l * (1 << (q - interval.end)) + r] += self.0.get(k);
            }
        }
        new_state_vector.normalize();
        
        let new_state = State::try_from(new_state_vector).expect("Could not create remaning state");

        (measured, new_state)
    }
    

    pub fn measure_partial_leave_state(&mut self, interval: Range<usize>) -> usize {
        let q = self.num_qubits();
        
        let mut prob_prefix_sum = Vec::new();

        let mut prob = 0.0;
        for m in 0..(1 << interval.len()) {
            for l in 0..(1 << interval.start) {
                for r in 0..(1 << (q - interval.end)) {
                    let k = (l << (q - interval.start)) + (m << (q - interval.end)) + r;
                    prob += self.0.get(k).modulus_squared();
                }
            }
            prob_prefix_sum.push(prob);
        }

        let mut measured = (1 << interval.len());
        while measured == (1 << interval.len()){
            let random_u64 = random::<u64>().min(u64::MAX - 1);
            let random_sample = (random_u64 as f64) / (u64::MAX as f64);
    
            measured = prob_prefix_sum.binary_search_by(|probe| {
                probe.partial_cmp(&random_sample).unwrap().then(std::cmp::Ordering::Greater)
            }).unwrap_err();
        }
        
        //Zero out states that don't match measurement
        for m in 0..(1 << interval.len()) {
            if m == measured { continue; }
            for l in 0..(1 << interval.start) {
                for r in 0..(1 << (q - interval.end)) {
                    let k = (l << (q - interval.start)) + (m << (q - interval.end)) + r;
                    self.0.data[k] = C64::ZERO;
                }
            }
            prob_prefix_sum.push(prob);
        }
        self.0.normalize();

        measured
    }
}

impl TryFrom<Vector<C64>> for State {
    type Error = ();
    fn try_from(value: Vector<C64>) -> Result<Self, Self::Error> {
        if !value.dim().is_power_of_two() {
            return Err(())
        }

        let mut sum = value.iter().fold(0.0, |acc, cur| acc + cur.modulus_squared());
        if (sum - 1.0).abs() < f64::EPSILON * 10.0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dynamic::vector::Vector;
    use crate::complex::*;
    use super::State;

    #[test]
    fn test_construction() {
        let a = Vector::from_iter((1..2).map(|n| C64::new(n as f64, 0.0)), Some(1));
        let b = Vector::from_iter((0..2).map(|_| C64::new(1.0 / 2.0.sqrt(),0.0)), Some(2));
        let c = Vector::<C64>::zero(16);
        let d = dvec64![2; 1,3;2; 3,-2];

        assert!(TryInto::<State>::try_into(a).is_ok());
        assert!(TryInto::<State>::try_into(b).is_ok());
        assert!(TryInto::<State>::try_into(c).is_err());
        assert!(TryInto::<State>::try_into(d).is_err());
    }
}