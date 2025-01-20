// use faer_evd::*;
// use faer_core::Parallelism;
// use num_complex::Complex;

use crate::complex::*;
use super::vector::*;

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign};
#[derive(Clone, PartialEq, Debug)]

//These being const make an emulator nearly impossible in Rust, I can't possible know the size of the matrices at compile time.
// Gonna split this up into a dynamic implementation vs static.
pub struct Matrix<const M: usize, const N: usize, F: Field> {
    pub data: [[Complex<F>;N];M]
}

pub trait MatrixT {}

impl<const M: usize, F: Field> MatrixT for Matrix<M,M,F> {}

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

    pub fn is_hermitian(&self) -> bool {
        if N != M { return false };
        let mut is_hermitian = true;
        for i in 0..N {
            for j in i..M {
                is_hermitian &= self.data[i][j] == self.data[j][i].conjugate()
            }
        }
        is_hermitian
    }

    pub fn fuzzy_equals(&self, rhs: &Self) -> bool {
        self.data.iter().flatten().zip(rhs.data.iter().flatten()).all(|(a,b)| a.fuzzy_equals(*b))
    }

    pub fn is_unitary(&self) -> bool {
        if M != N { return false; }

        let adj = self.as_adjoint();
        let a = &adj * self;
        let b = self * &adj;
        
        a.fuzzy_equals(&Matrix::<N,N,F>::eye()) && b.fuzzy_equals(&Matrix::<M,M,F>::eye())
    }

    pub fn tensor_product<const M2:usize , const N2: usize>(&self, rhs: &Matrix<M2, N2, F>) -> Matrix<{M * M2}, {N * N2}, F> {
        let mut data = [[Complex::<F>::default(); {N * N2}]; {M * M2}];

        for i in 0..M {
            for j in 0..N {
                for i2 in 0..M2 {
                    for j2 in 0..N2 {
                        data[i * M2 + i2][j * N2 + j2] = self.data[i][j] * rhs.data[i2][j2]
                    }
                }
            }
        }

        Matrix {
            data
        }

    }

    // Might just implement this myself
    // pub fn eigenpairs_hermitian(&self) {
    //     assert!(self.is_hermitian());

    //     let stack_req = compute_hermitian_evd_req(N, ComputeVectors::Yes, Parallelism::None, SymmetricEvdParams::default());
    // }
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
        for row in self.data.iter_mut(){
            for entry in row.iter_mut(){
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

#[macro_export]
macro_rules! mat64 {
    [$([$($r:literal $(+)? $($i:literal i)?),* ]),*] => {
        Matrix::new([$(
            [$({
                let i = 0.0;
                $(
                   let i = $i as f64;
                )?
                Complex::new($r as f64, i)
            }),*]
        ),*])
    };
}

#[macro_export]
macro_rules! mat32 {
    [$([$($r:literal $(+)? $($i:literal i)?),* ]),*] => {
        Matrix::new([$(
            [$({
                let mut i = 0.0;
                $(
                    i = $i as f32;
                )?
                Complex::new($r as f32, i)
            }),*]
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

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::new([[Complex::new(3.0, 2.0), Complex::new(0.0,0.0), Complex::new(5.0,-6.0)],
                             [Complex::new(1.0,0.0),Complex::new(4.0,2.0), Complex::new(0.0, 1.0)],
                             [Complex::new(4.0, -1.0), Complex::new(0.0, 0.0), Complex::new(4.0,0.0)]]);
        let b = Matrix::new([[Complex::new(5.0,0.0), Complex::new(2.0, -1.0), Complex::new(6.0, -4.0)],
                             [Complex::new(0.0, 0.0), Complex::new(4.0, 5.0), Complex::new(2.0,0.0)],
                             [Complex::new(7.0, -4.0), Complex::new(2.0,7.0), Complex::new(0.0, 0.0)]]);
        
        let ab = Matrix::new([[Complex::new(26.0,-52.0), Complex::new(60.0, 24.0), Complex::new(26.0,0.0)],
                              [Complex::new(9.0, 7.0), Complex::new(1.0, 29.0), Complex::new(14.0,0.0)],
                              [Complex::new(48.0, -21.0), Complex::new(15.0, 22.0), Complex::new(20.0, -22.0)]]);

        assert_eq!(&a * &b, ab);
    }

    #[test]
    fn test_unary_operators() {
       let a = mat64![[7, 6 + 5 i],[6 - 5 i, -3]];

       assert!(a.is_hermitian());
       assert!(!a.is_unitary());

       let u = Matrix::new([[c64!(1 + 1 i) / 2.0, c64!(0 + 1 i) / 3.0.sqrt(), c64!(3 + 1 i)/(2.0 * 15.0.sqrt())],
                            [c64!(-1)/2.0, c64!(1) / 3.0.sqrt(), c64!(4 + 3 i) / (2.0 * 15.0.sqrt())],
                            [c64!(1.0)/2.0, c64!(0 - 1 i) / 3.0.sqrt(), c64!(0 + 5 i) / (2.0 * 15.0.sqrt())]]);
        assert!(u.is_unitary());
        assert!(!u.is_hermitian());

    }

    #[test]
    fn test_tensor_product() {
        let a = mat64![[1,2],[0,1]];
        let b = mat64![[3,2],[-1,0]];
        let c = mat64![[6,5],[3,2]];
        
        let left = a.tensor_product(&b).tensor_product(&c);
        let right = a.tensor_product(&(b.tensor_product(&c)));

        assert!(left.fuzzy_equals(&right));

        let pre = a.as_adjoint().tensor_product(&b.as_adjoint());
        let post = a.tensor_product(&b).as_adjoint();
        assert!(pre.fuzzy_equals(&post));

        let av = vec64![1.0 + 3 i, 3.0];
        let bv = vec64![2.0 - 2.0 i, 1.0];

        let pre = &a.tensor_product(&b) * &av.tensor_product(&bv);
        let post = (&a * &av).tensor_product(&(&b * &bv));
        assert!(pre.fuzzy_equals(&post));

    }

}