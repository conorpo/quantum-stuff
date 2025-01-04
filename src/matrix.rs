use crate::complex::*;
use crate::vector::*;

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign};
#[derive(Clone, PartialEq, Debug)]
struct Matrix<const M: usize, const N: usize, F: Field> {
    pub data: [[Complex<F>;N];M]
}

impl<const M: usize, const N: usize, F: Field> Matrix<M,N,F> {
    pub fn new(data: [[Complex<F>;N];M]) -> Self {
        Self {
            data
        }
    }

    pub fn eye() -> Self
    {
        let mut data = [[Complex::zero();N];M];
        for i in 0..N.min(M) {
            data[i][i] = Complex::one()
        }
        Self {
            data
        }
    }

    pub fn as_transpose(&self) -> Matrix<N,M,F> {
        let mut data = [[Complex::zero();M];N];

        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[j][i];
            }
        }

        Matrix::<N,M,F> {
            data
        } 
    }

    pub fn as_conjugate(&self) -> Self {
        let mut data = [[Complex::zero();N];M];
        for i in 0..M {
            for j in 0..N {
                data[i][j] = self.data[i][j].conjugate();
            }
        }

        Self {
            data
        }
    }

    pub fn as_adjoint(&self) -> Matrix<N,M,F> {
        let mut data = [[Complex::zero();M];N];

        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[j][i].conjugate();
            }
        }

        Matrix::<N,M,F> {
            data
        } 
    }
}

impl<const M: usize, const N: usize, F: Field> Add<&Self> for Matrix<M,N,F> {
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

impl<const M: usize, const N: usize, F: Field> AddAssign<&Self> for Matrix<M,N,F> {
    fn add_assign(&mut self, rhs: &Self) {
        for (r, row) in self.data.iter_mut().enumerate() {
            for (c,entry) in row.iter_mut().enumerate() {
                entry.add_assign(rhs.data[r][c]);
            }
        }
    }
}

impl<const M: usize, const N: usize, F: Field> Neg for Matrix<M,N,F> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for (r, row) in self.data.iter_mut().enumerate() {
            for (c,entry) in row.iter_mut().enumerate() {
                *entry = entry.neg()
            }
        }
        self
    }
}

impl<const M: usize, const N: usize, F: Field> Sub<&Self> for Matrix<M,N,F> {
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

impl<const M: usize, const N: usize, F: Field> SubAssign<&Self> for Matrix<M,N,F> {
    fn sub_assign(&mut self, rhs: &Self) {
        for (r, row) in self.data.iter_mut().enumerate() {
            for (c,entry) in row.iter_mut().enumerate() {
                entry.sub_assign(rhs.data[r][c]);
            }
        }
    }
}

//Scalar Multiplication
impl<const M: usize, const N: usize, F: Field> Mul<Complex<F>> for Matrix<M,N,F> {
    type Output = Self;
    fn mul(mut self, rhs: Complex<F>) -> Self::Output {
        for row in self.data.iter_mut() {
            for entry in row.iter_mut() {
                entry.mul_assign(rhs);
            }
        }
        self
    }
}

impl<const M: usize, const N: usize, F: Field> MulAssign<Complex<F>> for Matrix<M,N,F> {
    fn mul_assign(&mut self, rhs: Complex<F>) {
        for row in self.data.iter_mut() {
            for entry in row.iter_mut() {
                entry.mul_assign(rhs);
            }
        }
    }
}

//Action on Vectors
impl<const M: usize, const N: usize, F: Field> Mul<&Vector<N,F>> for &Matrix<M,N,F>  {
    type Output = Vector<M,F>;

    fn mul(self, rhs: &Vector<N,F>) -> Vector<M,F> {
        let mut data = [Complex::zero(); M];
        for (r, row) in self.data.iter().enumerate() {
            data[r] = row.iter().zip(rhs.data.iter()).fold(Complex::zero(),|acc, (m, v)| acc + *m * *v);
        }
        Self::Output {
           data
        }
    }
}

//Matrix Multiplication
impl<const M: usize, const N: usize, const P: usize, F: Field> Mul<&Matrix<N,P,F>> for &Matrix<M,N,F> {
    type Output = Matrix<M,P,F>;

    fn mul(self, rhs: &Matrix<N,P,F>) -> Self::Output {
        let mut data = [[Complex::zero();P];M];

        for c in 0..P {
            for r in 0..M {
                data[r][c] = self.data[r].iter().enumerate().fold(Complex::zero(), |acc, (n, cur)| {
                    acc + *cur * rhs.data[n][c]
                })
            }
        }

        Self::Output {
            data
        }
    }
}

macro_rules! mat64 {
    [$([$($r:literal + $i:literal i),* ]),*] => {
        Matrix::new([$(
            [$(Complex::new($r as f64, $i as f64)),*]
        ),*])
    };
}

#[cfg(test)]
mod tests {
    use crate::complex::*;
    use super::*;

    #[test]
    fn test_vector_space() {
        let a = Matrix::new([[Complex::new(1.0,-1.0), Complex::new(3.0,0.0)],[Complex::new(2.0,2.0), Complex::new(4.0, 1.0)]]);
        assert_eq!(a.clone() + &a, a.clone() * Complex::new(2.0,0.0));
        assert_eq!(-a.clone() - &a, a * Complex::new(-2.0, 0.0));
    }

    fn test_matrix_multiplication() {
        let a = Matrix::new([[Complex::new(3.0, 2.0), Complex::new(0.0,0.0), Complex::new(5.0,-6.0)],
                             [Complex::new(1.0,0.0),Complex::new(4.0,2.0), Complex::new(0.0, 1.0)],
                             [Complex::new(4.0, -1.0), Complex::new(0.0, 0.0), Complex::new(4.0,0.0)]]);
        let b = Matrix::new([[Complex::new(5.0,0.0), Complex::new(2.0, -1.0), Complex::new(6.0, -4.0)],
                             [Complex::new(0.0, 0.0), Complex::new(4.0, 5.0), Complex::new(2.0,0.0)],
                             [Complex::new(7.0, -4.0), Complex::new(2.0,7.0), Complex::new(0.0, 0.0)]]);
        
        let ab = Matrix::new([[Complex::new(26.0,-52.0), Complex::new(60.0, 24.0), Complex::new(26.0,0.0)],
                              [Complex::new(9.0, 7.0), Complex::new(1.0, 29.0), Complex::new(14.0,0.0)],
                              [Complex::new(48.0, -21.0), Complex::new(15.0, 22.0), Complex::new(20.0, 22.0)]]);

        assert_eq!(&a * &b, ab);
    }

    fn test_unary_operators() {
        // let a = mat64![[2.0 + ]];
    }
}