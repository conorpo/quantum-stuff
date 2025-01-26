// use faer_evd::*;
// use faer_core::Parallelism;
// use num_complex::Complex;

use crate::complex::*;
use super::vector::*;

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign};

//These being const make an emulator nearly impossible in Rust, I can't possible know the size of the matrices at compile time.
// Gonna split this up into a dynamic implementation vs static.
#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<F: Real> {
    dim: (usize, usize),
    data: Vec<Complex<F>>
}

impl<F: Real> Matrix<F> {
    pub fn from_rows(iter: impl Iterator<Item = Vector<F>>, rows_hint: Option<usize>) -> Result<Self, ()> {
        let mut row_iter = iter.peekable();
        let (mut m, n) = (0, row_iter.peek().map(|row| row.dim()).unwrap_or(0));

        let mut data = Vec::with_capacity(rows_hint.unwrap_or(0) * n);

        for row in row_iter {
            if row.dim() != n { return Err(()); }
            m += 1;

            for entry in row.iter() {
                data.push(*entry);
            }
        }

        Ok(Self {
            dim: (m, n),
            data
        })
    }

    // pub fn from_grid()
    pub fn eye(n: usize) -> Self
    {
        let mut data = vec![Complex::zero(); n * n];
        let mut i = 0;
        while i < n*n {
            data[i] = Complex::one();
            i += n + 1;
        }

        Self {
            dim: (n,n),
            data
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Complex<F> {
        self.data[r * self.dim.1 + c]
    }

    pub fn transpose(&self) -> Self {
        let (m,n) = self.dim;
        let mut data = Vec::with_capacity(n*m);

        for c in 0..n {
            for r in 0..m {
                data.push(self.get(r,c));
            }
        }

        Self {
            dim: (n, m),
            data
        } 
    }

    pub fn conjugate(mut self) -> Self {
        for entry in self.data.iter_mut() {
            *entry = entry.conjugate();
        }
        self
    }

    pub fn adjoint(&self) -> Self {
        self.transpose().conjugate()
    }

    pub fn is_square(&self) -> bool {
        self.dim.0 == self.dim.1
    }

    pub fn is_hermitian(&self) -> bool {
        if !self.is_square() { return false; }

        let mut is_hermitian = true;
        for r in 0..self.dim.0 {
            for c in r..self.dim.1 {
                is_hermitian &= self.get(r,c).conjugate() == self.get(c,r);
            }
        }
        is_hermitian
    }

    pub fn dim(&self) -> (usize, usize) {
        self.dim
    }

    pub fn fuzzy_equals(&self, rhs: &Self) -> bool {
        self.dim() == rhs.dim() && self.data.iter().zip(rhs.data.iter()).all(|(a,b)| a.fuzzy_equals(*b))
    }

    pub fn is_identity(&self) -> bool {
        if !self.is_square() { return false; }

        self.data.iter().enumerate().filter(|(_, &entry)| !entry.fuzzy_equals(Complex::zero())).all(|(idx, entry)| {
            entry.fuzzy_equals(Complex::one()) && idx / self.dim.1 == idx % self.dim.1 
        })
    }

    pub fn row_iter(&self) -> VecIter<'_, false, F> {
        VecIter::<'_, false, F> {
            mat: self,
            index: 0
        }
    }

    pub fn col_iter(&self) -> VecIter<'_, true, F> {
        VecIter::<'_, true, F> {
            mat: self,
            index: 0
        }
    }

    pub fn is_unitary(&self) -> bool {
        if !self.is_square() { return false; }

        let adj = self.clone().adjoint();
        let b = (self * &adj).unwrap();
        let a = (&adj * self).unwrap();
        
        a.is_identity() && b.is_identity()
    }

    pub fn tensor_product(&self, rhs: &Self) -> Self {
        let (m1, n1) = self.dim;
        let (m2, n2) = rhs.dim();

        let mut data = Vec::with_capacity(m1 * n1 * m2 * n2);

        for r1 in 0..m1 {
            for c1 in 0..n1 {
                for r2 in 0 ..m2 {
                    for c2 in 0..n2 {
                        data.push(self.get(r1,c1) * rhs.get(r2,c2));
                    }
                }
            }
        }        

        Self {
            dim: (m1 * m2, n1 * n2),
            data
        }
        
    }

    // Might just implement this myself
    // pub fn eigenpairs_hermitian(&self) {
    //     assert!(self.is_hermitian());

    //     let stack_req = compute_hermitian_evd_req(N, ComputeVectors::Yes, Parallelism::None, SymmetricEvdParams::default());
    // }
}

impl<const M: usize, const N: usize, F: Real> From<[[Complex<F>; N];M]> for Matrix<F> {
    fn from(arr: [[Complex<F>; N];M]) -> Self {
        let mut data = Vec::with_capacity(N * M);
        let dim = (M, N);

        for r in 0..M {
            for c in 0..N {
                data.push(arr[r][c]);
            }
        }

        Self {
            dim,
            data,
        }
    }
}

pub struct VecIter<'a, const col_order: bool,  F: Real> {
    mat: &'a Matrix<F>,
    index: usize
}

impl<'a, F:Real> Iterator for VecIter<'a,false, F> {
    type Item = Vector<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let (m, n) = self.mat.dim();
        if self.index >= m { return None; }

        let row_vec = Vector::from_iter((0..n).map(|i| self.mat.get(self.index, i)), Some(n));

        self.index += 1;

        Some(row_vec)
    }
}

impl<'a, F:Real> Iterator for VecIter<'a,true, F> {
    type Item = Vector<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let (m, n) = self.mat.dim();
        if self.index >= n { return None; }

        let row_vec = Vector::from_iter((0..m).map(|i| self.mat.get(i, self.index)), Some(m));

        self.index += 1;

        Some(row_vec)
    }
}

impl<F: Real> Add<&Self> for Matrix<F> {
    type Output = Result<Self, ()>;

    fn add(mut self, rhs: &Self) -> Self::Output {
        if self.dim() != rhs.dim() { return Err(()); }

        for (i, entry) in self.data.iter_mut().enumerate() {
            entry.add_assign(rhs.data[i]);
        }

        Ok(self)
    }
}

impl<F: Real> AddAssign<&Self> for Matrix<F> {
    fn add_assign(&mut self, rhs: &Self) {
        debug_assert_eq!(self.dim(), rhs.dim());

        for (i, entry) in self.data.iter_mut().enumerate() {
            entry.add_assign(rhs.data[i]);
        }
    }
}

impl<F: Real> Neg for Matrix<F> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for entry in self.data.iter_mut(){
            *entry = entry.neg()
        }
        self
    }
}

impl<F: Real> Sub<&Self> for Matrix<F> {
    type Output = Result<Self,()>;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        if self.dim() != rhs.dim() { return Err(()); }

        for (i, entry) in self.data.iter_mut().enumerate() {
            entry.sub_assign(rhs.data[i]);
        }

        Ok(self)
    }
}

impl<F: Real> SubAssign<&Self> for Matrix<F> {
    fn sub_assign(&mut self, rhs: &Self) {
        debug_assert_eq!(self.dim(), rhs.dim());

        for (i, entry) in self.data.iter_mut().enumerate() {
            entry.sub_assign(rhs.data[i]);
        }
    }
}

//Scalar Multiplication
impl<F: Real> Mul<Complex<F>> for Matrix<F> {
    type Output = Self;
    fn mul(mut self, rhs: Complex<F>) -> Self::Output {
        for entry in self.data.iter_mut() {
            entry.mul_assign(rhs);
        }
        self
    }
}

impl<F: Real> MulAssign<Complex<F>> for Matrix<F> {
    fn mul_assign(&mut self, rhs: Complex<F>) {
        for entry in self.data.iter_mut() {
            entry.mul_assign(rhs);
        }
    }
}

//Action on Vectors
impl<F: Real> Mul<&Vector<F>> for &Matrix<F>  {
    type Output = Result<Vector<F>,()>;

    fn mul(self, rhs: &Vector<F>) -> Self::Output {
        if self.dim().1 != rhs.dim() { return Err(()); }

        let row_iter = self.row_iter();

        Ok(Vector::from_iter(
            row_iter.map(|row| row.dot(rhs).unwrap()), Some(self.dim().0)
        ))
    }
}

//Matrix Multiplication
impl<F: Real> Mul<Self> for &Matrix<F> {
    type Output = Result<Matrix<F>, ()>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.dim().1 != rhs.dim().0 { return Err(()) ;}

        let row_iter = self.row_iter();
        
        Ok(Matrix::<F>::from_rows(
            row_iter.map(|row| {
                Vector::from_iter(
                    rhs.col_iter().map(|col| {
                        row.dot(&col).unwrap()
                    }), 
                    Some(rhs.dim().1)
                )
            }), 
            Some(self.dim().0)
        ).unwrap())
    }
}

#[macro_export]
macro_rules! dmat64 {
    [$([$($r:literal $(+)? $($i:literal i)?),* ]),*] => {
        Matrix::from([$(
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
macro_rules! dmat32 {
    [$([$($r:literal $(+)? $($i:literal i)?),* ]),*] => {
        Matrix::from([$(
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
        let a = Matrix::from([[Complex::new(1.0,-1.0), Complex::new(3.0,0.0)],[Complex::new(2.0,2.0), Complex::new(4.0, 1.0)]]);
        assert_eq!((a.clone() + &a).unwrap(), a.clone() * Complex::new(2.0,0.0));
        assert_eq!((-a.clone() - &a).unwrap(), a * Complex::new(-2.0, 0.0));
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::from([[Complex::new(3.0, 2.0), Complex::new(0.0,0.0), Complex::new(5.0,-6.0)],
                             [Complex::new(1.0,0.0),Complex::new(4.0,2.0), Complex::new(0.0, 1.0)],
                             [Complex::new(4.0, -1.0), Complex::new(0.0, 0.0), Complex::new(4.0,0.0)]]);
        let b = Matrix::from([[Complex::new(5.0,0.0), Complex::new(2.0, -1.0), Complex::new(6.0, -4.0)],
                             [Complex::new(0.0, 0.0), Complex::new(4.0, 5.0), Complex::new(2.0,0.0)],
                             [Complex::new(7.0, -4.0), Complex::new(2.0,7.0), Complex::new(0.0, 0.0)]]);
        
        let ab = Matrix::from([[Complex::new(26.0,-52.0), Complex::new(60.0, 24.0), Complex::new(26.0,0.0)],
                              [Complex::new(9.0, 7.0), Complex::new(1.0, 29.0), Complex::new(14.0,0.0)],
                              [Complex::new(48.0, -21.0), Complex::new(15.0, 22.0), Complex::new(20.0, -22.0)]]);

        assert_eq!((&a * &b).unwrap(), ab);
    }

    #[test]
    fn test_unary_operators() {
       let a = dmat64![[7, 6 + 5 i],[6 - 5 i, -3]];

       assert!(a.is_hermitian());
       assert!(!a.is_unitary());

       let u = Matrix::from([[c64!(1 + 1 i) / 2.0, c64!(0 + 1 i) / 3.0.sqrt(), c64!(3 + 1 i)/(2.0 * 15.0.sqrt())],
                            [c64!(-1)/2.0, c64!(1) / 3.0.sqrt(), c64!(4 + 3 i) / (2.0 * 15.0.sqrt())],
                            [c64!(1.0)/2.0, c64!(0 - 1 i) / 3.0.sqrt(), c64!(0 + 5 i) / (2.0 * 15.0.sqrt())]]);
        assert!(u.is_unitary());
        assert!(!u.is_hermitian());

    }

    #[test]
    fn test_tensor_product() {
        let a = dmat64![[1,2],[0,1]];
        let b = dmat64![[3,2],[-1,0]];
        let c = dmat64![[6,5],[3,2]];
        
        let left = a.tensor_product(&b).tensor_product(&c);
        let right = a.tensor_product(&(b.tensor_product(&c)));

        assert!(left.fuzzy_equals(&right));

        let pre = a.adjoint().tensor_product(&b.adjoint());
        let post = a.tensor_product(&b).adjoint();
        assert!(pre.fuzzy_equals(&post));

        let av = dvec64![1.0 + 3 i, 3.0];
        let bv = dvec64![2.0 - 2.0 i, 1.0];

        let pre = (&a.tensor_product(&b) * &av.tensor_product(&bv)).unwrap();
        let post = (&a * &av).unwrap().tensor_product(&(&b * &bv).unwrap());
        assert!(pre.fuzzy_equals(&post));

    }

}