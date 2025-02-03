use core::f64;

use super::matrix::*;
use crate::complex::*;

pub struct Operator(Matrix<C64>);

impl Operator {
    pub fn get(&self) -> &Matrix<C64> {
        &self.0
    }

    pub fn dim(&self) -> usize {
        self.0.dim().0
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
        let mut mat = Matrix::zeroes(og_n * 2);
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
}

impl TryFrom<Matrix<C64>> for Operator {
    type Error = ();
    fn try_from(value: Matrix<C64>) -> Result<Self, Self::Error> {
        Ok(Operator(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::{complex::*};
    use crate::dynamic::matrix::*;
    use super::*;

    #[test]
    fn test_construction() {
        let theta = 4.0;
        let phase_shift = [[C64::ONE, C64::ZERO], [C64::ZERO, C64::new(0.0, theta).exp()]];
        let mat = Matrix::from(phase_shift);
        let op: Operator = mat.try_into().unwrap();
    }
}