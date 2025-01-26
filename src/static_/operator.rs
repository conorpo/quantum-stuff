use crate::complex::*;
use super::state::*;

use std::{array, ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign}};
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

    // pub const IDENTITY: &'static Self = &{
    //     let mut data = [[F::ZERO;N];N];
    //     let mut i = 0;
    //     while i < N {
    //         data[i][i] = F::ONE;
    //         i += 1;
    //     }
        
    //     Self {
    //         data
    //     }
    // };

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

    pub fn is_unitary(&self) -> bool {
        let adj = self.as_adjoint();
        let a = &adj * self;
        let b = self * &adj;
        
        a.fuzzy_equals(&Self::IDENTITY) && b.fuzzy_equals(&Self::IDENTITY)
    }

    pub fn tensor_product<const N2: usize>(&self, rhs: &Operator<N2, F>) -> Operator<{N * N2}, F> {
        let mut new_operator = Operator::<{N * N2}, F>::ZERO;

        for i in 0..N {
            for j in 0..N {
                for i2 in 0..N2 {
                    for j2 in 0..N2 {
                        new_operator.data[i * N2 + i2][j * N2 + j2] = self.data[i][j] * rhs.data[i2][j2]
                    }
                }
            }
        }

        new_operator
    }

    // Might just implement this myself
    // pub fn eigenpairs_hermitian(&self) {
    //     assert!(self.is_hermitian());

    //     let stack_req = compute_hermitian_evd_req(N, ComputeVectors::Yes, Parallelism::None, SymmetricEvdParams::default());
    // }
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

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut data = [[F::ZERO;N];N];

        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    data[i][j].add_assign(self.data[i][k] * rhs.data[k][j]);
                }
            }
        }

        Self::Output {
            data
        }
    }
}

static NOT : Operator<2, C64> = Operator::new([[C64::ZERO, C64::ONE],
                                           [C64::ONE, C64::ZERO]]);

impl Operator<2, C64> {
    const NOT: &'static Self = &NOT;
}

impl Operator<2, C32> {
    const H : Self = {
        const ENTRY: f32 = 1.0 / std::f32::consts::SQRT_2;
        Self::new([[C32::new(ENTRY, 0.0), C32::new(ENTRY, 0.0)],
                        [C32::new(ENTRY, 0.0), C32::new(-ENTRY, 0.0)]])
    };
}

impl Operator<2, C64> {
    const H : Self = {
        const ENTRY: f64 = 1.0 / std::f64::consts::SQRT_2;
        Self::new([[C64::new(ENTRY, 0.0), C64::new(ENTRY, 0.0)],
                        [C64::new(ENTRY, 0.0), C64::new(-ENTRY, 0.0)]])
    };
}

impl<F: Complex> Operator<4, F> {
    const CNOT: Self = Self::new([[F::ONE, F::ZERO, F::ZERO, F::ZERO],
                                 [F::ZERO, F::ONE, F::ZERO, F::ZERO],
                                 [F::ZERO, F::ZERO, F::ZERO, F::ONE],
                                 [F::ZERO, F::ZERO, F::ONE, F::ZERO]]);
}

impl Operator<2, C32> {
    const PHASE_SHIFT: fn(f32) -> Self = |theta| {
        let mut mat = Self::new([[C32::ONE, C32::ZERO],
                                 [C32::ZERO, C32::ZERO]]);
        mat.data[1][1] = C32::new(theta.exp(), 0.0);
        mat
    };
}

impl Operator<2, C64> {
    const PHASE_SHIFT: fn(f64) -> Self = |theta| {
        let mut mat = Self::new([[C64::ONE, C64::ZERO],
                                 [C64::ZERO, C64::ZERO]]);
        mat.data[1][1] = C64::new(theta.exp(), 0.0);
        mat
    };
}