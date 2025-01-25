use crate::dynamic::matrix::*;
use crate::complex::*;

#[derive(Copy, Clone, Debug)]
pub enum PrimitiveGate {
    H,
    R(f64),
    I(usize),
    CNOT
}

impl PrimitiveGate {
    pub fn get_operator(self) -> Matrix<f64> {
        match self {
            PrimitiveGate::H => {
                let h_element: f64 = 1.0 / 2.0f64.sqrt();
                Matrix::from([[Complex::new(h_element, 0.0), Complex::new(h_element, 0.0)],
                                            [Complex::new(h_element, 0.0), Complex::new(-h_element, 0.0)]])
            },
            PrimitiveGate::R(theta) => {
                Matrix::from([[Complex::one(), Complex::zero()],
                              [Complex::zero(), Complex::new(f64::exp(theta), 0.0)]])
            },
            PrimitiveGate::I(n) => {
                Matrix::eye(n)
            },
            PrimitiveGate::CNOT => {
                dmat64![[1,0,0,0],
                        [0,1,0,0],
                        [0,0,0,1],
                        [0,0,1,0]]
            }
        }
    } 
}