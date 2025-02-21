use core::f64;

use super::matrix::*;
use crate::complex::*;

#[derive(Clone)]
pub struct Gate(Matrix<C64>);

impl Gate {
    pub fn get(&self) -> &Matrix<C64> {
        &self.0
    }

    pub fn dim(&self) -> usize {
        self.0.dim().0
    }

    pub fn tensor_product(&self, rhs: &Self) -> Self {
        Self (self.0.tensor_product(&rhs.0))
    }

    pub fn identity(n: usize) -> Self {
        Self (Matrix::eye(n))
    }

    pub fn hadamard() -> Self {
        let entry = C64::new(1.0 / f64::consts::SQRT_2, 0.0);
        Self(Matrix::from([[entry,entry],[entry, -entry]]))
    }

    pub fn not() -> Self {
        Self(dmat64![[0;1],[1;0]])
    }

    pub fn cnot() -> Self {
        Self::controlled(Self::not())
    }
    
    pub fn phase_shift(theta: f64) -> Self {
        let mut mat = dmat64![[1;0],[0;1]];
        *mat.get_mut(1, 1) = C64::new(0.0, theta).exp();
        Self(mat)
    }

    pub fn pauli_x() -> Self {
        Self::not()
    }
    
    pub fn pauli_y() -> Self {
        Self(dmat64![[0;0,-1],[0,1;0]])
    }

    pub fn pauli_z() -> Self {
        Self(dmat64![[1;0],[0;-1]])
    }

    pub fn swap() -> Self {
        Self(dmat64![[1;0;0;0],[0;0;1;0],[0;1;0;0],[0;0;0;1]])
    }

    pub fn fredkin() -> Self {
        Self::controlled(Self::swap())
    }

    pub fn controlled(og: Self) -> Self {
        let og_n = og.dim();
        let mut mat = Matrix::zeroes(og_n * 2, og_n * 2);
        for i in 0..og_n {
            *mat.get_mut(i,i) = C64::ONE;
        }
        
        for r in 0..og_n {
            for c in 0..og_n {
                *mat.get_mut(og_n + r, og_n + c) = og.0.get(r, c);
            }
        }

        Self(mat)
    }

    pub fn create_oracle(input_bits: usize, output_bits: usize, f: impl Fn(usize) -> usize) -> Self {
        let size = 1 << (input_bits + output_bits);
        let mut mat: Matrix<C64> = Matrix::zeroes(size,size);
        for x in 0..(1 << input_bits) {
            let f_x = f(x);
            for y in 0..(1 << output_bits) {
                *mat.get_mut((x << output_bits) + (y ^ f_x), (x << output_bits) + y) = C64::ONE;
            }
        }
        Self::try_from(mat).unwrap()
    }

    pub fn create_oracle_unchecked(input_bits: usize, output_bits: usize, f: impl Fn(usize) -> usize) -> Self {
        let size = 1 << (input_bits + output_bits);
        let mut mat: Matrix<C64> = Matrix::zeroes(size,size);
        for x in 0..(1 << input_bits) {
            let f_x = f(x);
            for y in 0..(1 << output_bits) {
                *mat.get_mut((x << output_bits) + (y ^ f_x), (x << output_bits) + y) = C64::ONE;
            }
        }
        unsafe { Self::from_matrix_unchecked(mat) }
    }

    /// # Safety
    /// Nothing unsafe about this, simply want to disuade usage.
    pub unsafe fn from_matrix_unchecked(mat: Matrix<C64>) -> Self {
        Self(mat)
    }
}

impl TryFrom<Matrix<C64>> for Gate {
    type Error = ();
    fn try_from(value: Matrix<C64>) -> Result<Self, Self::Error> {
        if value.is_unitary() {
            Ok(Gate(value))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{complex::*};
    
    use super::*;

    #[test]
    fn test_construction() {
        let theta = 4.0;
        let phase_shift = [[C64::ONE, C64::ZERO], [C64::ZERO, C64::new(0.0, theta).exp()]];
        let mat = Matrix::from(phase_shift);
        let op: Gate = mat.try_into().unwrap();
    }
}