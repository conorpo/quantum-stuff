use std::array;

#[macro_use]
use crate::complex::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Vector<const N: usize, F: Field> {
    pub data: [Complex<F>; N]
}

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
}
