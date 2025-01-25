use std::array;

use super::matrix::*;
use crate::complex::*;

pub trait GateBase {
    type Output;
}

impl<const m: usize, const n: usize> GateBase for Matrix<m,n> {
    type Output = &'static Self;
}


pub trait NOTGate: GateBase {
    fn not() -> Self::Output;
}

static NOT_GATE: Matrix<2,2> = mat64![[0,1],[1,0]];

impl NOTGate for Matrix<2,2> {
    fn not() -> Self::Output {
       &NOT_GATE
    }
}

static CNOT_GATE: Matrix<4,4> = mat64![[1,0,0,0],
                                       [0,1,0,0],
                                       [0,0,0,1],
                                       [0,0,1,0]];

pub trait CNOTGate : GateBase {
    fn cnot() -> Self::Output;
}

impl CNOTGate for Matrix<4,4> {
    fn cnot() -> Self::Output {
        &CNOT_GATE
    }
}

static HADAMARD_GATE: Matrix<2,2> = {
    const ENTRY: f64 = 1.0 / std::f64::consts::SQRT_2;
    Matrix::new([[Complex::new(ENTRY,0.0),  Complex::new(ENTRY, 0.0)],
                 [Complex::new(ENTRY,0.0),  Complex::new(-ENTRY, 0.0)]])
};

pub trait HadamardGate: GateBase {
    fn h() -> Self::Output;
}

impl HadamardGate for Matrix<2,2> {
    fn h() -> Self::Output {
        &HADAMARD_GATE
    }
}

pub trait PhaseShift: GateBase {
    fn phase_shift(theta: f64) -> Self;
}

impl PhaseShift for Matrix<2,2> {
    fn phase_shift(theta: f64) -> Self {
        let mut mat = Self::eye();
        mat.data[1][1] = Complex::new(theta.exp(),0.0);
        mat
    }

}
