#[macro_use]
use crate::complex::*;
use std::slice::Iter;

#[derive(Clone, PartialEq, Debug)]
pub struct Vector<F: Real> {
    data: Vec<Complex<F>>,
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

// MARK: Vector
impl<F: Real> Vector<F> {
    pub fn zero(n: usize) -> Self {
        Self {
            data: vec![Complex::zero(); n]
        }
    }

    pub fn iter(&self) -> Iter<'_, Complex<F>> {
        self.data.iter()
    }

    pub fn from_iter(iter: impl Iterator<Item = Complex<F>>, size_hint: Option<usize>) -> Self {
        let mut data = Vec::with_capacity(size_hint.unwrap_or_default());
        data.extend(iter);

        Self {
            data
        }
    }

    pub fn dot(&self, rhs: &Self) -> Result<Complex<F>, ()>{
        if rhs.dim() == self.dim() {
            let mut result = Complex::zero();
            for i in 0..self.dim() {
                result += self.data[i].conjugate() * rhs.data[i];
            }
            Ok(result)
        } else {
            Err(())
        }
    }

    pub fn deterministic(n: usize, state: usize) -> Result<Self, ()> {
        if state >= n {
            return Err(());
        }

        let mut vec = Self::zero(n);
        vec.data[state] = Complex::one();
        Ok(vec)
    }

    pub fn dim(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Complex<F> {
        self.data[index]
    }

    pub fn norm(&self) -> F {
        let self_dot_self = self.dot(&self).unwrap();
        debug_assert_eq!(self_dot_self.i, F::default());
        self_dot_self.r.sqrt()
    }

    pub fn norm_squared(&self) -> F {
        self.dot(&self).unwrap().r
    }

    //TODO: Check if need to clone
    pub fn distance(&self, other: &Self) -> F {
        let mut dif_vec = self.clone();
        dif_vec -= other;
        dif_vec.norm()
    }

    pub fn fuzzy_equals(&self, rhs: &Self) -> bool {
        self.data.iter().zip(rhs.data.iter()).all(|(a,b)| a.fuzzy_equals(*b))
    }

    pub fn tensor_product(&self, rhs: &Self) -> Self {
        let mut data = Vec::with_capacity(self.dim() * rhs.dim());
        for a in self.data.iter().copied() {
            for b in self.data.iter().copied() {
                data.push(a*b);
            }
        }

        Self {
            data
        }
    }

    //Entry wise modulus squared
    pub fn probabilities(&self) -> Vec<F> {
        self.data.iter().copied().map(|c| (c.conjugate() * c).r).collect()
    }
}

impl<F: Real> AddAssign<&Self> for Vector<F> {
    fn add_assign(&mut self, rhs: &Self) {
        debug_assert_eq!(self.dim(), rhs.dim());
        for (i, comp) in self.data.iter_mut().enumerate() {
            *comp += rhs.data[i];
        }
    }
}


impl<F: Real> Add<&Self> for Vector<F> {
    type Output = Result<Self, ()>;

    fn add(mut self, rhs: &Self) -> Self::Output {
        if self.dim() != rhs.dim() {return Err(()); }

        for (i, elem) in self.data.iter_mut().enumerate() {
            elem.add_assign(rhs.get(i));
        }
        
        Ok(self)
    }
}

impl<F: Real> SubAssign<&Self> for Vector<F> {
    fn sub_assign(&mut self, rhs: &Self) {
        debug_assert_eq!(self.dim(), rhs.dim());
        for (i, comp) in self.data.iter_mut().enumerate() {
            comp.sub_assign(rhs.data[i]);
        }
    }
}

impl<F: Real> Sub<&Self> for Vector<F> {
    type Output = Result<Self, ()>;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        if self.dim() != rhs.dim() {return Err(()); }
        for (i, comp) in self.data.iter_mut().enumerate() {
            comp.sub_assign(rhs.data[i]);
        }
        Ok(self)
    }
}

impl<F: Real> Neg for Vector<F> {
    type Output = Vector<F>;

    fn neg(mut self) -> Self::Output {
        for comp in self.data.iter_mut() {
            *comp = -*comp;
        }
        self
    }
}

impl<F: Real> MulAssign<Complex<F>> for Vector<F> {
    fn mul_assign(&mut self, rhs: Complex<F>) {
        for comp in self.data.iter_mut() {
            comp.mul_assign(rhs);
        }
    }
}


impl<F: Real> Mul<Complex<F>> for Vector<F> {
    type Output = Self;
    fn mul(mut self, rhs: Complex<F>) -> Self::Output {
        for comp in self.data.iter_mut() {
            comp.mul_assign(rhs);
        }
        self
    }
}

#[macro_export]
macro_rules! dvec64 {
    [$($r:literal $(+)? $($i:literal i)?),* ] => {
        Vector::from_iter(
            [$({
                let mut i = 0.0;
                $(
                    i = $i as f64;
                )?
                Complex::new($r as f64, i)
            }),*].into_iter(), None
        )
    };
}

#[macro_export]
macro_rules! dvec32 {
    [$($r:literal $(+)? $($i:literal i)?),* ] => {
        Vector::from_iter(
            [$({
                let mut i = 0.0;
                $(
                    i = $i as f32;
                )?
                Complex::new($r as f32, i)
            }),*].into_iter(), None
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

        let mut va = Vector::<f64>::from_iter((0..4).map(|_| a), Some(4));
        let mut vb = Vector::<f64>::from_iter((0..4).map(|_| b), Some(4));
        let mut vc = Vector::<f64>::from_iter((0..4).map(|_| c), Some(4));

        assert_eq!((vb.clone() + &va).unwrap(), vc);
        assert_eq!(-va.clone(), vc);
        
        va *= Complex::new(-2.0,0.0);
        assert_eq!(va, vb);
    }

    #[test]
    fn test_inner_product() {
        let a = dvec64![1.0 - 1.0 i, 3.0];
        assert!(a.dot(&a).unwrap().r > 0.0);

        let b = dvec64![0.0, 0.0];
        assert_eq!(b.dot(&b).unwrap(), Complex::zero());

        let mut a = dvec64![1.0 + 2.0 i, -2.0 - 3.0 i];
        let mut b = dvec64![1.0 - 2.0 i, 2.0 + 3.0 i];
        let c = dvec64![2.0 + 3.0 i, 3.0 - 2.0 i];

        assert!(a.dot(&b).unwrap() == b.dot(&a).unwrap().conjugate());
        let b_c = (b.clone() + &c).unwrap();
        assert!(a.dot(&b_c).unwrap() == a.dot(&b).unwrap() + a.dot(&c).unwrap());

        let a_dot_b = a.dot(&b).unwrap();
        b *= Complex::new(2.0,1.0);
        assert_eq!(a.dot(&b).unwrap(), a_dot_b * Complex::new(2.0,1.0));

        let a_dot_c = a.dot(&c).unwrap();
        a *= Complex::new(2.0,1.0);
        assert_eq!(a.dot(&c).unwrap(), a_dot_c * Complex::new(2.0,1.0).conjugate());
    }

    #[test]
    fn test_norm_and_distance() {
        let a = dvec64![3, -6, 2];
        assert_eq!(a.norm(), 7.0);
        let c = c64!(2.0 + 1 i);
        let a = a * c;
        assert_eq!(a.norm(), 7.0 * c.modulus()); //Respects Scalar Multiplication

        let a = dvec64![3,1,2];
        let b = dvec64![2,2,-1];
        assert_eq!(a.distance(&b), 11.0.sqrt()); 
        assert_eq!(a.distance(&a), 0.0);
        assert_eq!(a.distance(&b), b.distance(&a)); //Symmetric
    }

    #[test]
    fn test_tensor_product() {
        let a = dvec64![2,3];
        let b = dvec64![4,6,3];

        let ab = dvec64![8,12,6,12,18,9];

        assert!(a.tensor_product(&b).fuzzy_equals(&ab));

        let c =  c64!(2 + -3.5 i);
        let a = a * c;
        assert!(a.tensor_product(&b).fuzzy_equals(&(ab * c)));


    }
}