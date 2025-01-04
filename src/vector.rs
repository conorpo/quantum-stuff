use std::array;

use crate::complex::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Vector<const N: usize, F: Field> {
    pub data: [Complex<F>; N]
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

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
            *comp += rhs.data[i];
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

#[cfg(test)]
mod tests {
    use std::array;

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
}
