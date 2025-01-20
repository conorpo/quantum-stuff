use std::array;

#[macro_use]
use crate::complex::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Vector<const N: usize, F: Field> {
    pub data: [Complex<F>; N]
}

pub trait VectorT {}

impl<const N: usize, F: Field> VectorT for Vector<N, F> {}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

// MARK: Vector
impl<const N: usize, F: Field> Vector<N,F> {
    pub fn new(components: [Complex<F>; N]) -> Self {
        Self {
            data: components
        }
    }

    pub fn zero() -> Self {
        Self {
            data: array::from_fn(|_| Complex::zero())
        }
    }   

    pub fn dot(&self, rhs: &Self) -> Complex<F> {
        let mut result = Complex::zero();
        for i in 0..N {
            result += self.data[i].conjugate() * rhs.data[i];
        }
        result
    }

    pub fn norm(&self) -> F {
        let self_dot_self = self.dot(&self);
        debug_assert_eq!(self_dot_self.i, F::default());
        self_dot_self.r.sqrt()
    }

    pub fn distance(&self, other: &Self) -> F {
        let dif_vec = self.clone() - other;
        dif_vec.norm()
    }

    pub fn fuzzy_equals(&self, rhs: &Self) -> bool {
        self.data.iter().zip(rhs.data.iter()).all(|(a,b)| a.fuzzy_equals(*b))
    }

    pub fn tensor_product<const N_2:usize>(&self, rhs: &Vector<N_2,F>) -> Vector<{N * N_2}, F> {
        let mut data = [Complex::<F>::default(); {N * N_2}];
        for i in 0..N {
            for j in 0..N_2 {
                data[i * N_2 + j] = self.data[i] * rhs.data[j];
            }
        }

        Vector {
            data
        }
    }

    //Entry wise modulus squared
    pub fn probabilities(&self) -> [F; N] {
        let mut res = [F::default(); N];
        for (i, entry) in self.data.iter().enumerate() {
            res[i] = (*entry * entry.conjugate()).r;
        }
        res
    }
}

impl<const N: usize, F: Field> AddAssign<&Self> for Vector<N, F> {
    fn add_assign(&mut self, rhs: &Self) {
        for (i, comp) in self.data.iter_mut().enumerate() {
            *comp += rhs.data[i];
        }
    }
}


impl<const N: usize, F: Field> Add<&Self> for Vector<N, F> {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        for (i, comp) in self.data.iter_mut().enumerate() {
            comp.add_assign(rhs.data[i]);
        }
        self
    }
}

impl<const N: usize, F: Field> SubAssign<&Self> for Vector<N, F> {
    fn sub_assign(&mut self, rhs: &Self) {
        for (i, comp) in self.data.iter_mut().enumerate() {
            comp.sub_assign(rhs.data[i]);
        }
    }
}

impl<const N: usize, F: Field> Sub<&Self> for Vector<N,F> {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        for (i, comp) in self.data.iter_mut().enumerate() {
            comp.sub_assign(rhs.data[i]);
        }
        self
    }
}

impl<const N: usize, F: Field> Neg for Vector<N, F> {
    type Output = Vector<N, F>;

    fn neg(mut self) -> Self::Output {
        for comp in self.data.iter_mut() {
            *comp = -*comp;
        }
        self
    }
}

impl<const N: usize, F: Field> MulAssign<Complex<F>> for Vector<N, F> {
    fn mul_assign(&mut self, rhs: Complex<F>) {
        for comp in self.data.iter_mut() {
            comp.mul_assign(rhs);
        }
    }
}


impl<const N: usize, F: Field> Mul<Complex<F>> for Vector<N, F> {
    type Output = Self;
    fn mul(mut self, rhs: Complex<F>) -> Self::Output {
        for comp in self.data.iter_mut() {
            comp.mul_assign(rhs);
        }
        self
    }
}

macro_rules! vec64 {
    [$($r:literal $(+)? $($i:literal i)?),* ] => {
        Vector::new(
            [$({
                let mut i = 0.0;
                $(
                    i = $i as f64;
                )?
                Complex::new($r as f64, i)
            }),*]
        )
    };
}

macro_rules! vec32 {
    [$($r:literal $(+)? $($i:literal i)?),* ] => {
        Vector::new(
            [$({
                let mut i = 0.0;
                $(
                    i = $i as f32;
                )?
                Complex::new($r as f32, i)
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
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(-2.0, -4.0);
        let c = a + b;

        let mut va = Vector::<4,f64>::new(array::from_fn(|_| a));
        let vb = Vector::new(array::from_fn(|_| b));
        let vc = Vector::new(array::from_fn(|_| c));

        assert_eq!(vb.clone() + &va, vc);
        assert_eq!(-va.clone(), vc);
        
        va *= Complex::new(-2.0,0.0);
        assert_eq!(va, vb);
    }

    #[test]
    fn test_inner_product() {
        let a = vec64![1.0 - 1.0 i, 3.0];
        assert!(a.dot(&a).r > 0.0);

        let b = vec64![0.0, 0.0];
        assert_eq!(b.dot(&b), Complex::zero());

        let mut a = vec64![1.0 + 2.0 i, -2.0 - 3.0 i];
        let mut b = vec64![1.0 - 2.0 i, 2.0 + 3.0 i];
        let c = vec64![2.0 + 3.0 i, 3.0 - 2.0 i];

        assert!(a.dot(&b) == b.dot(&a).conjugate());
        let b_c = b.clone() + &c;
        assert!(a.dot(&b_c) == a.dot(&b) + a.dot(&c));

        let a_dot_b = a.dot(&b);
        b *= Complex::new(2.0,1.0);
        assert_eq!(a.dot(&b), a_dot_b * Complex::new(2.0,1.0));

        let a_dot_c = a.dot(&c);
        a *= Complex::new(2.0,1.0);
        assert_eq!(a.dot(&c), a_dot_c * Complex::new(2.0,1.0).conjugate());
    }

    #[test]
    fn test_norm_and_distance() {
        let a = vec64![3, -6, 2];
        assert_eq!(a.norm(), 7.0);
        let c = c64!(2.0 + 1 i);
        let a = a * c;
        assert_eq!(a.norm(), 7.0 * c.modulus()); //Respects Scalar Multiplication

        let a = vec64![3,1,2];
        let b = vec64![2,2,-1];
        assert_eq!(a.distance(&b), 11.0.sqrt()); 
        assert_eq!(a.distance(&a), 0.0);
        assert_eq!(a.distance(&b), b.distance(&a)); //Symmetric
    }

    #[test]
    fn test_tensor_product() {
        let a = vec64![2,3];
        let b = vec64![4,6,3];

        let ab = vec64![8,12,6,12,18,9];

        assert!(a.tensor_product(&b).fuzzy_equals(&ab));

        let c =  c64!(2 + -3.5 i);
        let a = a * c;
        assert!(a.tensor_product(&b).fuzzy_equals(&(ab * c)));


    }
}
