
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
use std::fmt::{Display,Debug};
use std::f64::consts::PI;
const TAU: f64 = 2.0 * PI;

// Trait for types allowed as complex number components MARK: Field
pub trait Real: Sized
{
    // Only way to get around this fuzzy equivalnce is to abandon IEEE float representation, not doing that for this toy implementation.
    const EPSILON: Self;
    const ZERO: Self;
    const ONE: Self;
    fn sqrt(self) -> Self;
}

impl Real for f32 {
    const EPSILON: Self = f32::EPSILON * 10.0;
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Real for f64 {
    const EPSILON: Self = f64::EPSILON * 10.0;
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

//MARK: Complex
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct C32 {
    pub r: f32,
    pub i: f32
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct C64 {
    pub r: f64,
    pub i: f64,
}

pub trait Complex: Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign + Neg<Output = Self> + Mul<Output = Self> + MulAssign + Div<Output = Self> + DivAssign + Copy + Clone + PartialEq
where Self: Sized
{
    type RealType: Real + PartialEq + Debug;
    const ZERO: Self;
    const ONE: Self; 
    const I: Self;

    fn get_r(&self) -> Self::RealType; 
    fn get_i(&self) -> Self::RealType;
    fn modulus(self) -> Self::RealType;
    fn conjugate(self) -> Self;
    fn fuzzy_equals(self, rhs: Self) -> bool;
    fn pow(self, exp: u32) -> Self;
    fn real(r: Self::RealType) -> Self;
}

impl C32 {
    pub const fn new(r: f32, i: f32) -> Self {
        Self {
            r,
            i
        }
    }
}

impl C64 {
    pub const fn new(r: f64, i: f64) -> Self {
        Self {
            r,
            i
        }
    }
}

impl Complex for C32 {
    type RealType = f32;
    const ZERO: Self = C32 { r: 0.0, i: 0.0};
    const ONE: Self = C32 { r: 1.0, i: 0.0};
    const I: Self = C32 { r: 0.0, i:-1.0};

    fn get_r(&self) -> Self::RealType {
        self.r
    }    

    fn get_i(&self) -> Self::RealType {
        self.i
    }

    fn conjugate(self) -> Self {
        Self {
            i: -self.i,
            ..self
        }
    }
    
    fn modulus(self) -> Self::RealType {
        (self.r * self.r + self.i * self.i).sqrt()
    }

    fn pow(self, mut exp: u32) -> Self {
        let mut res = Self::ONE;  // Presumably 1 + 0i
        let mut mult = self;                    // Base (the 'self' Complex number)
        while exp > 0 {
            // If the current bit (LSB) of 'exp' is 1, multiply 'res' by 'mult'
            if exp % 2 == 1 {
                res *= mult;
            }
            // Shift exponent to the right by 1 bit
            exp >>= 1;
            // Square 'mult' for the next iteration
            mult *= mult;
        }
        res
    }

    fn fuzzy_equals(self, rhs: Self) -> bool {
        (self.get_r() - rhs.get_r()).abs() < Self::RealType::EPSILON &&
        (self.get_i() - rhs.get_i()).abs() < Self::RealType::EPSILON
    }

    fn real(r: Self::RealType) -> Self {
        Self {
            r,
            i: 0.0
        }
    }
}

impl Complex for C64 {
    type RealType = f64;
    const ZERO: Self = C64 { r: 0.0, i: 0.0};
    const ONE: Self = C64 { r: 1.0, i: 0.0};
    const I: Self = C64 { r: 0.0, i:-1.0};

    fn get_r(&self) -> Self::RealType {
        self.r
    }    

    fn get_i(&self) -> Self::RealType {
        self.i
    }

    fn real(r: Self::RealType) -> Self {
        Self {
            r,
            i: 0.0
        }
    }


    fn conjugate(self) -> Self {
        Self {
            i: -self.i,
            ..self
        }
    }

    fn modulus(self) -> Self::RealType {
        (self.r * self.r + self.i * self.i).sqrt()
    }

    fn pow(self, mut exp: u32) -> Self {
        let mut res = Self::ONE;  // Presumably 1 + 0i
        let mut mult = self;                    // Base (the 'self' Complex number)
        while exp > 0 {
            // If the current bit (LSB) of 'exp' is 1, multiply 'res' by 'mult'
            if exp % 2 == 1 {
                res *= mult;
            }
            // Shift exponent to the right by 1 bit
            exp >>= 1;
            // Square 'mult' for the next iteration
            mult *= mult;
        }
        res
    }

    fn fuzzy_equals(self, rhs: Self) -> bool {
        (self.get_r() - rhs.get_r()).abs() < Self::RealType::EPSILON &&
        (self.get_i() - rhs.get_i()).abs() < Self::RealType::EPSILON
    }
}

impl From<f64> for C64 {
    fn from(val: f64) -> Self {
        Self {
            r: val,
            i: 0.0
        }
    }
}

impl From<i32> for C64 {
    fn from(value: i32) -> Self {
        Self {
            r: value as f64,
            i: 0.0
        }
    }
}


impl From<f32> for C32 {
    fn from(val: f32) -> Self {
        Self {
            r: val,
            i: 0.0
        }
    }
}

impl From<i16> for C32 {
    fn from(value: i16) -> Self {
        Self {
            r: value as f32,
            i: 0.0
        }
    }
}



// impl From<Complex<F>> for ComplexP<F> {
//     fn from(value: Complex<F>) -> Self {
//         Self {
//             rho: value.modulus(),
//             theta: value.i.atan2(value.r)
//         }
//     }
// }

// impl From<ComplexP<F>> for Complex<F> {
//     fn from(value: ComplexP<F>) -> Self {
//         let (sin, cos) = value.theta.sin_cos();
//         Self {
//             r: cos * value.rho,
//             i: sin * value.rho
//         }
//     }
// }

#[macro_export]
macro_rules! c32 {
    ($r: expr $(, $i: expr)?) => {
        {
            let mut r = $r as f32;
            let mut i = 0.0;
            $(
                i = $i as f32;
            )?

            C32::new(r,i)
        }
    }
}

#[macro_export]
macro_rules! c64 {
    ($r: expr $(, $i: expr)?) => {
        {
            let r = $r as f64;
            let i = 0.0;
            $(
                let i = $i as f64;
            )?

            C64::new(r,i)
        }
    }
}

impl Display for C32
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = if self.get_i() < 0.0 {
            '-'
        } else {
            '+'
        };

        f.write_fmt(format_args!("{} {op} {}i", self.r, self.i.abs()))
    }
}

// impl<F: Field> Display for ComplexP<F> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("({},{})", self.rho, <F as Into<f64>>::into(self.theta) % TAU))
//     }
// }

// MARK: Operations
impl Add<Self> for C32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}

impl Add<Self> for C64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}


impl AddAssign<Self> for C32{
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

impl AddAssign<Self> for C64 {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

impl Sub<Self> for C32 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            i: self.i - rhs.i
        }
    }
}


impl Sub<Self> for C64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            i: self.i - rhs.i
        }
    }
}

impl SubAssign<Self> for C32 {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.i -= rhs.i;
    }
}

impl SubAssign<Self> for C64 {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.i -= rhs.i;
    }
}

impl Neg for C32 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            r: -self.r,
            i: -self.i
        }
    }
}

impl Neg for C64 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            r: -self.r,
            i: -self.i
        }
    }
}

impl Mul<C32> for C32 {
    type Output = C32;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r
        }
    }
}

impl Mul<C64> for C64 {
    type Output = C64;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r
        }
    }
}

macro_rules! impl_scalar_left_mult {
    ($t:ty, $c:ty) => {
        impl Mul<$c> for $t {
            type Output = $c;
            fn mul(self, rhs: $c) -> Self::Output {
                Self::Output {
                    r: self * rhs.r,
                    i: self * rhs.i
                }
            }   
        }
    };
}

impl_scalar_left_mult!(f32, C32);
impl_scalar_left_mult!(f64, C64);

impl Mul<f32> for C32 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            r: self.r * rhs,
            i: self.i * rhs
        }
    }
}

impl MulAssign<C32> for C32 {
    fn mul_assign(&mut self, rhs: Self) {
        let real = self.r;
        self.r = self.r * rhs.r - self.i * rhs.i;
        self.i = real * rhs.i + self.i * rhs.r;
    }
}

impl MulAssign<f32> for C32 {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.i *= rhs;
    }
}

impl MulAssign<C64> for C64 {
    fn mul_assign(&mut self, rhs: Self) {
        let real = self.r;
        self.r = self.r * rhs.r - self.i * rhs.i;
        self.i = real * rhs.i + self.i * rhs.r;
    }
}

impl MulAssign<f64> for C64 {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.i *= rhs;
    }
}


impl Div<C32> for C32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let divisor = rhs.r * rhs.r + rhs.i * rhs.i;
        Self {
            r: (self.r * rhs.r + self.i * rhs.i) / divisor,
            i: (self.i * rhs.r - self.r * rhs.i) / divisor
        }
    }
}

impl Div<f32> for C32 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            r: self.r / rhs,
            i: self.i / rhs
        }
    }
}


impl Div<C64> for C64 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let divisor = rhs.r * rhs.r + rhs.i * rhs.i;
        Self {
            r: (self.r * rhs.r + self.i * rhs.i) / divisor,
            i: (self.i * rhs.r - self.r * rhs.i) / divisor
        }
    }
}

impl Div<f64> for C64 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            r: self.r / rhs,
            i: self.i / rhs
        }
    }
}


impl DivAssign<C32> for C32 {
    fn div_assign(&mut self, rhs: C32) {
        let divisor = rhs.r * rhs.r + rhs.i * rhs.i;
        let real = self.r;
        self.r = (self.r * rhs.r + self.i * rhs.i) / divisor;
        self.i = (self.i * rhs.r - real * rhs.i) / divisor
    }
}

impl DivAssign<f32> for C32{
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.i /= rhs;
    }
}

impl DivAssign<C64> for C64 {
    fn div_assign(&mut self, rhs: C64) {
        let divisor = rhs.r * rhs.r + rhs.i * rhs.i;
        let real = self.r;
        self.r = (self.r * rhs.r + self.i * rhs.i) / divisor;
        self.i = (self.i * rhs.r - real * rhs.i) / divisor
    }
}

impl DivAssign<f64> for C64{
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.i /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drill_1_1_1() {
        let a = C32::new(-3.0,1.0);
        let b = C32::new(2.0,-4.0);
        
        assert_eq!(a + b, C32::new(-1.0, -3.0));
        assert_eq!(a * b, C32::new(-2.0, 14.0));
    }

    #[test]
    fn test_div() {
        let a = C32::new(-2.0,1.0);
        let b = C32::new(1.0,2.0);

        assert_eq!(a / b, C32::new(0.0, 1.0));
    }

    #[test]
    fn exc_1_2_12() {
        let z = C32::new(3f32,2f32);

        assert_eq!(z * z.conjugate(), C32::new(z.modulus().powf(2.0),0.0))
    }

    // #[test]
    // fn exc_1_3_8() {
    //     let z = c64!(1 + -1.0 i);

    //     let z_polar: ComplexP<f64> = z.into();
    //     let C32 {r: ra, i: ia} = z;
    //     let C32 {r: rb, i: ib} = z_polar.into();

    //     assert!((ra-rb).abs() < f64::EPSILON * 2.0);
    //     assert!((ia-ib).abs() < f64::EPSILON * 2.0);

    //     assert_eq!(z.pow(5), Complex::new(-4.0, 4.0));
    // }
}
