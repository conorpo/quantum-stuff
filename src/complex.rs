
use std::ops::{Add, Mul, Neg, Sub, AddAssign, SubAssign, MulAssign, Div, DivAssign};
use std::fmt::Display;

// Trait for types allowed as complex number components MARK: Field
pub trait Field: Copy + PartialEq + Default + From<u8> + PartialOrd + Display + 
             Add<Self, Output = Self> + AddAssign<Self> + Sub<Self, Output = Self> + SubAssign<Self> + Neg<Output = Self> +
             Mul<Self, Output = Self> + MulAssign<Self> + Div<Self, Output = Self> + DivAssign<Self> 
{
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
}

impl Field for i32 {
    fn sqrt(self) -> Self {
        self.isqrt()
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

impl Field for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

impl Field for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

//MARK: Complex
#[derive(Clone, Copy, Hash, PartialEq, Debug)]
pub struct Complex<F: Field> {
    pub r: F,
    pub i: F,
}

impl<F: Field> Default for Complex<F> {
    fn default() -> Self {
        Self {
            r: F::default(),
            i: F::default()
        }
    }
}

impl<F: Field> Complex<F> {
    pub fn new(real: F, imaginary: F) -> Self {
        Self {
            r: real,
            i: imaginary
        }
    }

    pub fn identity_add() -> Self {
        Self::default()
    }

    pub fn identity_mul() -> Self {
        Self {
            r: 1.into(),
            i: F::default()
        }
    }

    pub fn conjugate(self) -> Self {
        Self {
            r: self.r,
            i: -self.i
        }
    }

    pub fn modulus(self) -> F {
        (self.r * self.r + self.i * self.i).sqrt()
    }
}

impl<F: Field> Display for Complex<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = if self.i < F::default() {
            '-'
        } else {
            '+'
        };

        f.write_fmt(format_args!("{} {op} {}i", self.r, self.i.abs()))
    }
}

// MARK: Operations
impl<F: Field> Add<Self> for Complex<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}

impl<F: Field> AddAssign<Self> for Complex<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

impl<F: Field> Sub<Self> for Complex<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            i: self.i - rhs.i
        }
    }
}

impl<F: Field> SubAssign<Self> for Complex<F> {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.i -= rhs.i;
    }
}

impl<F: Field> Mul<Complex<F>> for Complex<F> {
    type Output = Complex<F>;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r
        }
    }
}

macro_rules! impl_scalar_left_mult {
    ($t:ty) => {
        impl Mul<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn mul(self, rhs: Complex<$t>) -> Self::Output {
                Self::Output {
                    r: self * rhs.r,
                    i: self * rhs.i
                }
            }   
        }
    };
}

impl_scalar_left_mult!(i32);
impl_scalar_left_mult!(f32);
impl_scalar_left_mult!(f64);

impl<F: Field> Mul<F> for Complex<F> {
    type Output = Self;
    fn mul(self, rhs: F) -> Self {
        Self {
            r: self.r * rhs,
            i: self.i * rhs
        }
    }
}

impl<F: Field> MulAssign<Self> for Complex<F> {
    fn mul_assign(&mut self, rhs: Self) {
        let real = self.r;
        self.r = self.r * rhs.r - self.i * rhs.i;
        self.i = real * rhs.i + self.i * rhs.r;
    }
}

impl<F: Field> MulAssign<F> for Complex<F> {
    fn mul_assign(&mut self, rhs: F) {
        self.r *= rhs;
        self.i *= rhs;
    }
}

impl<F: Field> Div<F> for Complex<F> {
    type Output = Self;
    fn div(self, rhs: F) -> Self {
        Self {
            r: self.r / rhs,
            i: self.i / rhs
        }
    }
}

impl<F: Field> Div<Self> for Complex<F> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let divisor = rhs.r * rhs.r + rhs.i * rhs.i;
        Self {
            r: (self.r * rhs.r + self.i * rhs.i) / divisor,
            i: (self.i * rhs.r - self.r * rhs.i) / divisor
        }
    }
}

impl<F: Field> DivAssign<F> for Complex<F> {
    fn div_assign(&mut self, rhs: F) {
        self.r /= rhs;
        self.i /= rhs;
    }
}

impl<F: Field> DivAssign<Complex<F>> for Complex<F> {
    fn div_assign(&mut self, rhs: Complex<F>) {
        let divisor = rhs.r * rhs.r + rhs.i * rhs.i;
        let real = self.r;
        self.r = (self.r * rhs.r + self.i * rhs.i) / divisor;
        self.i = (self.i * rhs.r - real * rhs.i) / divisor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drill_1_1_1() {
        let a = Complex::new(-3,1);
        let b = Complex::new(2,-4);
        
        assert_eq!(a + b, Complex::new(-1, -3));
        assert_eq!(a * b, Complex::new(-2, 14));
    }

    #[test]
    fn test_div() {
        let a = Complex::new(-2.0,1.0);
        let b = Complex::new(1.0,2.0);

        assert_eq!(a / b, Complex::new(0.0, 1.0));
    }

    #[test]
    fn exc_1_2_12() {
        let z = Complex::new(3f32,2f32);

        assert_eq!(z * z.conjugate(), Complex::new(z.modulus().powf(2.0),0.0))
    }
}
