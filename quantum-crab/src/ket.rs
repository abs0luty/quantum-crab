use crate::{complex::Complex, matrix::Matrix};

pub struct Ket {
    inner: Matrix<Complex>,
}

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
