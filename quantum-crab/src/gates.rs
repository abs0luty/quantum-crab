use std::f64::consts::PI;

use crate::{complex::Complex, matrix::Matrix};
use num::{One, Zero};

type Gate = Matrix<Complex>;

#[inline]
pub fn identity2() -> Gate {
    matrix_real![[1, 0], [0, 1]]
}

#[inline]
pub fn identity(n: usize) -> Gate {
    Matrix::identity(n)
}

pub fn hadamard() -> Gate {
    matrix_real![[1, 1], [1, -1]] * Complex::from(1f64 / f64::sqrt(2f64))
}

pub fn phase_shift(phase: f64) -> Gate {
    matrix![
        [Complex::one(), Complex::zero()],
        [Complex::zero(), Complex::new_from_polar(1f64, phase)]
    ]
}

pub fn t() -> Gate {
    phase_shift(PI / 4f64)
}

pub fn pauli_x() -> Gate {
    matrix_real![[0, 1], [1, 0]]
}

pub fn pauli_y() -> Gate {
    matrix![
        [Complex::zero(), -Complex::i()],
        [Complex::i(), Complex::zero()]
    ]
}

pub fn pauli_z() -> Gate {
    matrix_real![[1, 0], [0, -1]]
}

pub fn swap() -> Gate {
    matrix_real![[1, 0, 0, 0], [0, 0, 1, 0], [0, 1, 0, 0], [0, 0, 0, 1]]
}
