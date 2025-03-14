use crate::complex::*;
use super::state::*;

use std::{array, ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign}};
#[derive(Clone, PartialEq, Debug)]


pub struct Operator<const N: usize, F: Complex> {
    pub data: [[F;N];N]
}


impl<const N: usize, F: Complex> Operator<N,F> {
    pub const fn zero() -> Self {
        Self {
            data: [[F::ZERO; N]; N]
        }
    }

    pub const fn eye() -> Self {
        let mut data = [[F::ZERO;N];N];
        let mut i = 0;
        while i < N {
            data[i][i] = F::ONE;
            i += 1;
        }
        
        Self {
            data
        }
    }

    pub const fn new(data: [[F;N];N]) -> Self {
        Self {
            data
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&F> {
        self.data.get(r).and_then(|row| row.get(c))
    }

    pub fn as_transpose(&self) -> Self {
        let mut data: [[F;N];N] = array::from_fn(|i| {
            array::from_fn(|j| {
                self.data[j][i]
            })
        });

        Self {
            data
        } 
    }

    pub fn conjugate(mut self) -> Self {
        for i in 0..N {
            for j in 0..N {
                self.data[i][j] = self.data[i][j].conjugate();
            }
        }

        self
    }

    pub fn as_adjoint(&self) -> Self {
        self.as_transpose().conjugate()
    }

    pub fn is_hermitian(&self) -> bool {
        let mut is_hermitian = true;
        for i in 0..N {
            for j in i..N {
                is_hermitian &= self.data[i][j] == self.data[j][i].conjugate()
            }
        }
        is_hermitian
    }

    pub fn fuzzy_equals(&self, rhs: &Self) -> bool {
        self.data.iter().flatten().zip(rhs.data.iter().flatten()).all(|(a,b)| a.fuzzy_equals(*b))
    }

    pub fn tensor_product<const N2: usize>(&self, rhs: &Operator<N2, F>) -> Operator<{N * N2}, F> {
        let mut data = [[F::ZERO; N * N2];N*N2];

        for i in 0..N {
            for j in 0..N {
                for i2 in 0..N2 {
                    for j2 in 0..N2 {
                        data[i * N2 + i2][j * N2 + j2] = self.data[i][j] * rhs.data[i2][j2]
                    }
                }
            }
        }

        Operator::<{N * N2}, F> {
            data
        }
    }

    pub fn expected_value(&self, state: &State<N,F>) -> Result<F::RealType, &'static str> {
        if !self.is_hermitian() { return Err("Observables need to be Hermitian matrices.")}
        let after = self * state;
        Ok(state.dot(&after).get_r())
    }

    pub fn variance(&self, state: &State<N,F>) -> Result<F::RealType, &'static str> {
        let expected = self.expected_value(state)?;

        let demeaned = -(Self::eye() * F::from_real(expected)) + self;
        let demeaned_squared = &demeaned * &demeaned;
        
        Ok(demeaned_squared.expected_value(state)?)

    }
}

impl<const N:usize> Operator<N, C64> {
    pub fn is_unitary(&self) -> bool {
        let adj = self.as_adjoint();
        let a = &adj * self;
        let b = self * &adj;
        
        a.fuzzy_equals(Self::IDENTITY) && b.fuzzy_equals(Self::IDENTITY)
    }
}

impl<const N:usize> Operator<N, C32> {
    pub fn is_unitary(&self) -> bool {
        let adj = self.as_adjoint();
        let a = &adj * self;
        let b = self * &adj;
        
        a.fuzzy_equals(Self::IDENTITY) && b.fuzzy_equals(Self::IDENTITY)
    }
}


impl<const N: usize, F: Complex> Add<&Self> for Operator<N,F> {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        for (r, row) in self.data.iter_mut().enumerate() {
            for (c,entry) in row.iter_mut().enumerate() {
                entry.add_assign(rhs.data[r][c]);
            }
        }
        self
    }
}

//Add Assign
impl<const N: usize, F: Complex> AddAssign<&Self> for Operator<N,F> {
    fn add_assign(&mut self, rhs: &Self) {
        for (r, row) in self.data.iter_mut().enumerate() {
            for (c,entry) in row.iter_mut().enumerate() {
                entry.add_assign(rhs.data[r][c]);
            }
        }
    }
}

impl<const N: usize, F: Complex> Sub<&Self> for Operator<N,F> {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        for (r, row) in self.data.iter_mut().enumerate() {
            for (c,entry) in row.iter_mut().enumerate() {
                entry.sub_assign(rhs.data[r][c]);
            }
        }
        self
    }
}

//Sub Assign
impl<const N: usize, F: Complex> SubAssign<&Self> for Operator<N,F> {
    fn sub_assign(&mut self, rhs: &Self) {
        for (r, row) in self.data.iter_mut().enumerate() {
            for (c,entry) in row.iter_mut().enumerate() {
                entry.sub_assign(rhs.data[r][c]);
            }
        }
    }
}

impl<const N: usize, F: Complex> Neg for Operator<N,F> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for row in self.data.iter_mut() {
            for entry in row.iter_mut() {
                *entry = entry.neg()
            }
        }
        self
    }
}

impl<const N: usize, F: Complex> Mul<F> for Operator<N,F> {
    type Output = Self;

    fn mul(mut self, rhs: F) -> Self::Output {
        for row in self.data.iter_mut() {
            for entry in row.iter_mut() {
                entry.mul_assign(rhs);
            }
        }
        self
    }
}

impl<const N: usize, F: Complex> Mul<&State<N,F>> for &Operator<N,F> {
    type Output = State<N,F>;

    fn mul(self, rhs: &State<N,F>) -> Self::Output {
        let mut data = array::from_fn(|i| {
            let mut sum = F::ZERO;
            for j in 0..N {
                sum.add_assign(self.data[i][j] * rhs.data[j]);
            }
            sum
        });

        State {
            data
        }
    }
}

impl<const N: usize, F: Complex> Mul<Self> for &Operator<N,F> {
    type Output = Operator<N,F>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [[F::ZERO;N];N];

        for (i, row) in self.data.iter().enumerate() {
            for j in 0..N {
                for k in 0..N {
                    data[i][j].add_assign(row[k] * rhs.data[k][j]);
                }
            }
        }

        Self::Output {
            data
        }
    }
}

//MARK: Macros
macro_rules! op64 {
    [$([$($r: expr $(, $i: expr)?);*]),*] => {
        Operator::new([$([
            $(c64!($r $(, $i)?)),*
        ]),*])
    };
}

macro_rules! op32 {
    [$([$($r: expr $(, $i: expr)?);*]),*] => {
        Operator::new([$([
            $(c32!($r $(, $i)?)),*
        ]),*])
    };
}




// MARK: GATES
//Yes these double static defenitions are kind of uglyW

impl<const N: usize> Operator<N, C64> {
    pub const IDENTITY: &'static Self = &{
        let mut data = [[C64::ZERO;N];N];
        let mut i = 0;
        while i < N {
            data[i][i] = C64::ONE;
            i += 1;
        }
        
        Self {
            data
        }
    };
}

impl<const N: usize> Operator<N, C32> {
    pub const IDENTITY: &'static Self = &{
        let mut data = [[C32::ZERO;N];N];
        let mut i = 0;
        while i < N {
            data[i][i] = C32::ONE;
            i += 1;
        }
        
        Self {
            data
        }
    };
}

impl Operator<2,C64> {
    pub const NOT: &'static Self = &{
        op64![[0;1],[1;0]]
    };

    pub const H: &'static Self = &{
        const ENTRY: f64 = 1.0 / std::f64::consts::SQRT_2;
        op64![[ENTRY,0; ENTRY,0],
                [ENTRY,0; -ENTRY,0]]
    };
}

impl Operator<2,C32> {
    pub const NOT: &'static Self = &{
        op32![[0;1],[1;0]]
    };

    pub const H: &'static Self = &{
        const ENTRY: f32 = 1.0 / std::f32::consts::SQRT_2;
        op32![[ENTRY,0; ENTRY,0],
                [ENTRY,0; -ENTRY,0]]
    };
}


impl Operator<4, C32> {
    pub const CNOT: &'static Self = &{
        op32![[1;0;0;0],
              [0;1;0;0],
              [0;0;0;1],
              [0;0;1;0]]
    };
}

impl Operator<4, C64> {
    pub const CNOT: &'static Self = &{
        op64![[1;0;0;0],
              [0;1;0;0],
              [0;0;0;1],
              [0;0;1;0]]
    };
}

impl Operator<2, C32> {
    pub fn phase_shift(theta: f32) -> Self {
        let mut mat = op32![[1;0],[0;1]];
        mat.data[1][1] = C32::new(theta.exp(), 0.0);
        mat
    }
}

impl Operator<2, C64> {
    pub fn phase_shift(theta: f64) -> Self {
        let mut mat = op64![[1;0],[0;1]];
        mat.data[1][1] = C64::new(theta.exp(), 0.0);
        mat
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn 
// }

/*
Observable Variance, Expected Value test

    fn ex_4_2_5() {
        let c: f64 = 2.0.sqrt() / 2.0;
        let state = State::new([Complex::new(c,0.0), Complex::new(0.0,c)]);
        let observable = mat64![[1, 0 - 1 i],
                                [0 + 1 i , 2]];
        
        let expectation = expected_value(&observable, &state).unwrap();
        println!("{expectation}");
        assert!((expectation - 2.5).abs() < f64::EPSILON * 100.0);

        let var = variance(&observable, &state).unwrap();
        println!("{var}");
        assert!((var - 0.25).abs() < f64::EPSILON);

    }
*/

