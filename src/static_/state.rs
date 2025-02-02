use std::random::random;
use std::array;

#[macro_use]
use crate::complex::*;
use std::slice::Iter;

#[derive(Clone, PartialEq, Debug)]
pub struct State<const N: usize, F: Complex> {
    pub data: [F; N]
}

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

// MARK: Vector
impl<const N: usize, F: Complex> State<N, F> {
    pub const fn zero() -> Self {
        Self {
            data: [F::ZERO; N]
        }
    }

    pub const fn new(components: [F; N]) -> Self {
        Self {
            data: components
        }
    }

    // //LSB willbe first qubit in state
    // pub fn from_bits<const NUM_BITS: usize>(initial_state: [u8; NUM_BITS]) -> Self
    // where [(); (2usize.pow(NUM_BITS as u32) == N) as usize - 1]: {
    //     let state = State::zero();
    //     state
    // }

    pub fn iter(&self) -> Iter<'_, F> {
        self.data.iter()
    }

    pub fn dot(&self, rhs: &Self) -> F {
        let mut result = F::ZERO;
        for i in 0..N {
            result += self.data[i].conjugate() * rhs.data[i];
        }
        result
    }

    pub fn norm(&self) -> F::RealType {
        let self_dot_self = self.dot(&self);
        debug_assert_eq!(self_dot_self.get_i(), F::RealType::ZERO);
        self_dot_self.get_r().sqrt()
    }

    pub fn distance(&self, other: &Self) -> F::RealType {
        let dif_vec = self.clone() - other;
        dif_vec.norm()
    }

    pub fn fuzzy_equals(&self, rhs: &Self) -> bool {
        self.data.iter().zip(rhs.data.iter()).all(|(a,b)| a.fuzzy_equals(*b))
    }

    pub fn tensor_product<const N_2:usize>(&self, rhs: &State<N_2, F>) -> State<{N * N_2}, F> {
        let mut new_state = State::<{N * N_2}, F>::zero();
        for i in 0..N {
            for j in 0..N_2 {
                new_state.data[i * N_2 + j] = self.data[i] * rhs.data[j];
            }
        }

        new_state
    }

    //Entry wise modulus squared
    pub fn probabilities(&self) -> [F::RealType; N] {
        let mut res = [F::RealType::ZERO; N];
        for (i, entry) in self.data.iter().enumerate() {
            res[i] = (*entry * entry.conjugate()).get_r();
        }
        res
    }
}

impl<const N: usize> State<N, C64> {
    pub fn measure(&self) -> usize {
        let rand: u32 = random();
        let sample = (rand as f64) / (u32::MAX as f64);
        let mut sum = 0.0;
        for (i, prob) in self.probabilities().iter().enumerate() {
            sum += prob;
            if sum > sample {
                return i;
            }
        }
        panic!("How did we get here.");
    }
}

impl<F: Complex> State<2, F> {
    pub fn qubit_zero() -> Self {
        Self::new([F::ONE, F::ZERO])
    }

    pub fn qubit_one() -> Self {
        Self::new([F::ZERO, F::ONE])
    }
}

impl<const N: usize, F: Complex> AddAssign<&Self> for State<N, F> {
    fn add_assign(&mut self, rhs: &Self) {
        for (i, comp) in self.data.iter_mut().enumerate() {
            *comp += rhs.data[i];
        }
    }
}


impl<const N: usize, F: Complex> Add<&Self> for State<N,F> {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        for (i, comp) in self.data.iter_mut().enumerate() {
            comp.add_assign(rhs.data[i]);
        }
        self
    }
}

impl<const N: usize, F: Complex> SubAssign<&Self> for State<N,F> {
    fn sub_assign(&mut self, rhs: &Self) {
        for (i, comp) in self.data.iter_mut().enumerate() {
            comp.sub_assign(rhs.data[i]);
        }
    }
}

impl<const N: usize, F: Complex> Sub<&Self> for State<N,F> {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        for (i, comp) in self.data.iter_mut().enumerate() {
            comp.sub_assign(rhs.data[i]);
        }
        self
    }
}

impl<const N: usize, F: Complex> Neg for State<N,F> {
    type Output = State<N,F>;

    fn neg(mut self) -> Self::Output {
        for comp in self.data.iter_mut() {
            *comp = -*comp;
        }
        self
    }
}

impl<const N: usize, F: Complex> MulAssign<F> for State<N,F> {
    fn mul_assign(&mut self, rhs: F) {
        for comp in self.data.iter_mut() {
            comp.mul_assign(rhs);
        }
    }
}


impl<const N: usize, F: Complex> Mul<F> for State<N,F> {
    type Output = Self;
    fn mul(mut self, rhs: F) -> Self::Output {
        for comp in self.data.iter_mut() {
            comp.mul_assign(rhs);
        }
        self
    }
}

#[macro_export]
macro_rules! state64 {
    [$($r:expr $(,$i:expr)?);* ] => {
        State::new(
            [$({
                c64!($r $(, $i)?)
            }),*]
        )
    };
}

#[macro_export]
macro_rules! state32 {
    [$($c: tt);* ] => {
        State::new(
            [$({
                c32!($c)
            }),*]
        )
    };
}



#[cfg(test)]
mod tests {
    use std::{array, vec};

    use crate::complex::*;
    use super::*;
    #[test]
    fn test_ops() {
        let a = C64::new(1.0, 2.0);
        let b = C64::new(-2.0, -4.0);
        let c = a + b;

        let mut va = State::<4, _>::new(array::from_fn(|_| a));
        let vb = State::new(array::from_fn(|_| b));
        let vc = State::new(array::from_fn(|_| c));

        assert_eq!(vb.clone() + &va, vc);
        assert_eq!(-va.clone(), vc);
        
        va *= C64::new(-2.0,0.0);
        assert_eq!(va, vb);
    }

    #[test]
    fn test_inner_product() {
        let a = state64![1,-1 ; 3.0];
        assert!(a.dot(&a).r > 0.0);

        let b = state64![0.0; 0.0];
        assert_eq!(b.dot(&b), C64::ZERO);

        let mut a = state64![1,2 ; -2, -3];
        let mut b = state64![1, -2; 2, 3];
        let c = state64![2, 3; 3, -2];

        assert!(a.dot(&b) == b.dot(&a).conjugate());
        let b_c = b.clone() + &c;
        assert!(a.dot(&b_c) == a.dot(&b) + a.dot(&c));

        let a_dot_b = a.dot(&b);
        b *= C64::new(2.0,1.0);
        assert_eq!(a.dot(&b), a_dot_b * C64::new(2.0,1.0));

        let a_dot_c = a.dot(&c);
        a *= C64::new(2.0,1.0);
        assert_eq!(a.dot(&c), a_dot_c * C64::new(2.0,1.0).conjugate());
    }

    #[test]
    fn test_norm_and_distance() {
        let a = state64![3; -6; 2];
        assert_eq!(a.norm(), 7.0);
        let c = c64!(2.0, 1);
        let a = a * c;
        assert_eq!(a.norm(), 7.0 * c.modulus()); //Respects Scalar Multiplication

        let a = state64![3;1;2];
        let b = state64![2;2;-1];
        assert_eq!(a.distance(&b), 11.0.sqrt()); 
        assert_eq!(a.distance(&a), 0.0);
        assert_eq!(a.distance(&b), b.distance(&a)); //Symmetric
    }

    #[test]
    fn test_tensor_product() {
        let a = state64![2;3];
        let b = state64![4;6;3];

        let ab = state64![8;12;6;12;18;9];

        assert!(a.tensor_product(&b).fuzzy_equals(&ab));

        let c =  c64!(2,-3.5);
        let a = a * c;
        assert!(a.tensor_product(&b).fuzzy_equals(&(ab * c)));


    }
}
