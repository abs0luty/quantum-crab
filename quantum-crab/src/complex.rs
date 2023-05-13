use core::fmt;
use num::{One, Zero};
use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

#[macro_export]
macro_rules! complex {
    [$re:expr, $imag:expr] => {
        $crate::complex::Complex::new($re as f64, $imag as f64)
    };
}

#[macro_export]
macro_rules! real {
    [$re:expr] => {
        $crate::complex::Complex::new($re as f64, 0f64)
    };
}

impl Default for Complex {
    fn default() -> Complex {
        Complex::zero()
    }
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }

    pub fn new_from_polar(r: f64, phi: f64) -> Complex {
        Complex {
            real: r * phi.cos(),
            imag: r * phi.sin(),
        }
    }

    pub fn norm(self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    pub fn conjugate(self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }

    pub fn i() -> Complex {
        complex![0, 1]
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        complex![self.real + rhs.real, self.imag + rhs.imag]
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        complex![
            self.real * rhs.real - self.imag * rhs.imag,
            self.real * rhs.imag + self.imag * rhs.real
        ]
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        complex![-self.real, -self.imag]
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Sum for Complex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = complex![0, 0];

        for c in iter {
            sum += c;
        }

        sum
    }
}

impl One for Complex {
    fn one() -> Self {
        real![1]
    }
}

impl Zero for Complex {
    fn zero() -> Self {
        complex![0, 0]
    }

    fn is_zero(&self) -> bool {
        self.real == 0f64 && self.imag == 0f64
    }
}

#[test]
fn complex() {
    assert_eq!(complex![1, 2], complex![1, 0] + complex![0, 2]);
    assert_eq!(complex![1, 0], complex![1, 2] - complex![0, 2]);
    assert_eq!(complex![14, 2], complex![1, 3] * complex![2, -4]);

    let mut a = complex![4, 1];
    a += complex![0, 3];
    a -= complex![4, 1];
    assert_eq!(complex![0, 3], a);
}
