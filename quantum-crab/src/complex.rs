//! Contains an implementation of complex number mathematics.
use core::fmt;
use num::{One, Zero};
use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Represents complex number.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Complex {
    /// Real part of the complex number.
    pub real: f64,
    /// Imaginary part of the complex number.
    pub imag: f64,
}

impl<T> From<T> for Complex
where
    T: Into<f64>,
{
    fn from(num: T) -> Self {
        Complex::new(num, 0)
    }
}

impl Default for Complex {
    fn default() -> Complex {
        Complex::zero()
    }
}

impl Complex {
    /// Constructs a complex number by its real and imaginary part.
    pub fn new<R, I>(real: R, imag: I) -> Complex
    where
        R: Into<f64>,
        I: Into<f64>,
    {
        Complex {
            real: real.into(),
            imag: imag.into(),
        }
    }

    /// Constructs a complex number by its polar coodinates (length & angle).
    ///
    /// ```
    /// use quantum_crab::complex::Complex;
    /// use float_cmp::approx_eq;
    /// use std::f64::consts::PI;
    ///  
    /// let b = Complex::new_from_polar(f64::sqrt(2f64), PI / 4f64);
    /// approx_eq!(f64, b.real, 1f64, ulps = 2);
    /// approx_eq!(f64, b.imag, 1f64, ulps = 2);
    /// ```
    pub fn new_from_polar<R, P>(r: R, phi: P) -> Complex
    where
        R: Into<f64>,
        P: Into<f64>,
    {
        let (r, phi) = (r.into(), phi.into());

        Complex {
            real: r * phi.cos(),
            imag: r * phi.sin(),
        }
    }

    /// Returns complex number's norm (length of the vector in complex space).
    ///
    /// ```
    /// use quantum_crab::complex::Complex;
    ///
    /// let c = Complex::new(3, 4);
    /// assert_eq!(c.norm(), 5f64);
    /// ```
    pub fn norm(self) -> f64 {
        f64::sqrt(self.real * self.real + self.imag * self.imag)
    }

    /// Returns complex number's conjugate.
    ///
    /// ```
    /// use quantum_crab::complex::Complex;
    ///
    /// let c = Complex::new(1, 2);
    /// assert_eq!(c.conjugate(), Complex::new(1, -2));
    /// ```
    pub fn conjugate(self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }

    /// Represents imaginary unit i or sqrt(-1).
    pub fn i() -> Complex {
        Complex::new(0, 1)
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
        Complex::new(self.real + rhs.real, self.imag + rhs.imag)
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
        Complex::new(
            self.real * rhs.real - self.imag * rhs.imag,
            self.real * rhs.imag + self.imag * rhs.real,
        )
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex::new(-self.real, -self.imag)
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
        let mut sum = Complex::zero();

        for c in iter {
            sum += c;
        }

        sum
    }
}

impl One for Complex {
    fn one() -> Self {
        1.into()
    }
}

impl Zero for Complex {
    fn zero() -> Self {
        Complex::new(0, 0)
    }

    fn is_zero(&self) -> bool {
        self.real == 0f64 && self.imag == 0f64
    }
}

#[cfg(test)]
mod tests {
    use crate::complex::Complex;
    use float_cmp::approx_eq;
    use std::f64::consts::PI;

    #[test]
    fn complex_number_test() {
        assert_eq!(Complex::new(1, 2), Complex::new(1, 0) + Complex::new(0, 2));
        assert_eq!(Complex::new(1, 0), Complex::new(1, 2) - Complex::new(0, 2));
        assert_eq!(
            Complex::new(14, 2),
            Complex::new(1, 3) * Complex::new(2, -4)
        );

        let mut a = Complex::new(4, 1);
        a += Complex::new(0, 3);
        a -= Complex::new(4, 1);
        assert_eq!(Complex::new(0, 3), a);

        let b = Complex::new_from_polar(f64::sqrt(2f64), PI / 4f64);
        approx_eq!(f64, b.real, 1f64, ulps = 2);
        approx_eq!(f64, b.imag, 1f64, ulps = 2);
    }
}
