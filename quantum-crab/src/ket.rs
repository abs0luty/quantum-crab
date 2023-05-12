use crate::{classical_register::ClassicalRegister, complex::Complex, matrix::Matrix};
use num::One;

#[derive(Debug)]
pub struct Ket {
    inner: Matrix<Complex>,
}

#[derive(Debug)]
pub struct Bra {
    inner: Matrix<Complex>,
}

impl From<Bra> for Ket {
    fn from(bra: Bra) -> Ket {
        Ket {
            inner: bra.inner.hermitian_transpose(),
        }
    }
}

impl From<Ket> for Bra {
    fn from(ket: Ket) -> Self {
        Bra {
            inner: ket.inner.hermitian_transpose(),
        }
    }
}

impl From<Matrix<Complex>> for Ket {
    fn from(matrix: Matrix<Complex>) -> Ket {
        assert_eq!(matrix.rows(), 1);
        Ket { inner: matrix }
    }
}

impl From<Matrix<Complex>> for Bra {
    fn from(matrix: Matrix<Complex>) -> Bra {
        assert_eq!(matrix.cols(), 1);
        Bra { inner: matrix }
    }
}

impl From<&ClassicalRegister> for Ket {
    fn from(register: &ClassicalRegister) -> Ket {
        let mut ket = Matrix::new_with_default_elems(1, register.width());
        ket.set(0, register.value() as usize, Complex::one());
        ket.into()
    }
}

impl From<&ClassicalRegister> for Bra {
    fn from(register: &ClassicalRegister) -> Self {
        let mut bra = Matrix::new_with_default_elems(register.width(), 1);
        bra.set(register.value() as usize, 0, Complex::one());
        bra.into()
    }
}
