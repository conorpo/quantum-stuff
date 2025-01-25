// use faer_evd::*;
// use faer_core::Parallelism;
// use num_complex::<f64>::Complex;

use crate::complex::*;
use super::vector::*;

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign};
#[derive(Clone, PartialEq, Debug)]

//These being const make an emulator nearly impossible in Rust, I can't possible know the size of the matrices at compile time.
// Gonna split this up into a dynamic implementation vs static.
pub struct Matrix<const M: usize, const N: usize> {
    pub data: [[Complex<f64>;N];M]
}

impl<const M: usize, const N: usize> Matrix<M,N> {
    pub const fn new(data: [[Complex<f64>;N];M]) -> Self {
        Self {
            data
        }
    }

    pub fn eye() -> Self
    {
        let mut data = [[Complex::<f64>::zero(); N]; M];

        for i in 0..N.min(M) {
            data[i][i] = Complex::<f64>::one();
        }

        Self {
            data
        }
    }

    pub fn as_transpose(&self) -> Matrix<N,M> {
        let mut data = [[Complex::<f64>::zero();M];N];

        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[j][i];
            }
        }

        Matrix::<N,M> {
            data
        } 
    }

    pub fn as_conjugate(&self) -> Self {
        let mut data = [[Complex::<f64>::zero();N];M];
        for i in 0..M {
            for j in 0..N {
                data[i][j] = self.data[i][j].conjugate();
            }
        }

        Self {
            data
        }
    }

    pub fn as_adjoint(&self) -> Matrix<N,M> {
        let mut data = [[Complex::<f64>::zero();M];N];

        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[j][i].conjugate();
            }
        }

        Matrix::<N,M> {
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
        
        a.fuzzy_equals(&Matrix::<N,N>::eye()) && b.fuzzy_equals(&Matrix::<M,M>::eye())
    }

    pub fn tensor_product<const M2:usize , const N2: usize>(&self, rhs: &Matrix<M2, N2>) -> Matrix<{M * M2}, {N * N2}> {
        let mut data = [[Complex::default(); {N * N2}]; {M * M2}];

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


impl<const M: usize, const N: usize> Add<&Self> for Matrix<M,N> {
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

impl<const M: usize, const N: usize> AddAssign<&Self> for Matrix<M,N> {
    fn add_assign(&mut self, rhs: &Self) {
        for (r, row) in self.data.iter_mut().enumerate() {
            for (c,entry) in row.iter_mut().enumerate() {
                entry.add_assign(rhs.data[r][c]);
            }
        }
    }
}

impl<const M: usize, const N: usize> Neg for Matrix<M,N> {
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

impl<const M: usize, const N: usize> Sub<&Self> for Matrix<M,N> {
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

impl<const M: usize, const N: usize> SubAssign<&Self> for Matrix<M,N> {
    fn sub_assign(&mut self, rhs: &Self) {
        for (r, row) in self.data.iter_mut().enumerate() {
            for (c,entry) in row.iter_mut().enumerate() {
                entry.sub_assign(rhs.data[r][c]);
            }
        }
    }
}

//Scalar Multiplication
impl<const M: usize, const N: usize> Mul<Complex<f64>> for Matrix<M,N> {
    type Output = Self;
    fn mul(mut self, rhs: Complex<f64>) -> Self::Output {
        for row in self.data.iter_mut() {
            for entry in row.iter_mut() {
                entry.mul_assign(rhs);
            }
        }
        self
    }
}

impl<const M: usize, const N: usize> MulAssign<Complex<f64>> for Matrix<M,N> {
    fn mul_assign(&mut self, rhs: Complex<f64>) {
        for row in self.data.iter_mut() {
            for entry in row.iter_mut() {
                entry.mul_assign(rhs);
            }
        }
    }
}

//Action on Vectors
impl<const M: usize, const N: usize> Mul<&Vector<N>> for &Matrix<M,N>  {
    type Output = Vector<M>;

    fn mul(self, rhs: &Vector<N>) -> Vector<M> {
        let mut data = [Complex::<f64>::zero(); M];
        for (r, row) in self.data.iter().enumerate() {
            data[r] = row.iter().zip(rhs.data.iter()).fold(Complex::<f64>::zero(),|acc, (m, v)| acc + *m * *v);
        }
        Self::Output {
           data
        }
    }
}

//Matrix Multiplication
impl<const M: usize, const N: usize, const P: usize> Mul<&Matrix<N,P>> for &Matrix<M,N> {
    type Output = Matrix<M,P>;

    fn mul(self, rhs: &Matrix<N,P>) -> Self::Output {
        let mut data = [[Complex::<f64>::zero();P];M];

        for c in 0..P {
            for r in 0..M {
                data[r][c] = self.data[r].iter().enumerate().fold(Complex::<f64>::zero(), |acc, (n, cur)| {
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
                Complex::<f64>::new($r as f64, i)
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
        let a = Matrix::new([[Complex::<f64>::new(1.0,-1.0), Complex::<f64>::new(3.0,0.0)],[Complex::<f64>::new(2.0,2.0), Complex::<f64>::new(4.0, 1.0)]]);
        assert_eq!(a.clone() + &a, a.clone() * Complex::<f64>::new(2.0,0.0));
        assert_eq!(-a.clone() - &a, a * Complex::<f64>::new(-2.0, 0.0));
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::new([[Complex::<f64>::new(3.0, 2.0), Complex::<f64>::new(0.0,0.0), Complex::<f64>::new(5.0,-6.0)],
                             [Complex::<f64>::new(1.0,0.0),Complex::<f64>::new(4.0,2.0), Complex::<f64>::new(0.0, 1.0)],
                             [Complex::<f64>::new(4.0, -1.0), Complex::<f64>::new(0.0, 0.0), Complex::<f64>::new(4.0,0.0)]]);
        let b = Matrix::new([[Complex::<f64>::new(5.0,0.0), Complex::<f64>::new(2.0, -1.0), Complex::<f64>::new(6.0, -4.0)],
                             [Complex::<f64>::new(0.0, 0.0), Complex::<f64>::new(4.0, 5.0), Complex::<f64>::new(2.0,0.0)],
                             [Complex::<f64>::new(7.0, -4.0), Complex::<f64>::new(2.0,7.0), Complex::<f64>::new(0.0, 0.0)]]);
        
        let ab = Matrix::new([[Complex::<f64>::new(26.0,-52.0), Complex::<f64>::new(60.0, 24.0), Complex::<f64>::new(26.0,0.0)],
                              [Complex::<f64>::new(9.0, 7.0), Complex::<f64>::new(1.0, 29.0), Complex::<f64>::new(14.0,0.0)],
                              [Complex::<f64>::new(48.0, -21.0), Complex::<f64>::new(15.0, 22.0), Complex::<f64>::new(20.0, -22.0)]]);

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