use crate::{classical_register::ClassicalRegister, complex::Complex, matrix::Matrix};
use std::cell::Cell;

#[derive(Debug)]
pub struct QuantumRegister {
    width: usize,
    statevector: Matrix<Complex>,
    collapsed: Cell<bool>,
}

impl QuantumRegister {
    #[inline]
    pub fn statevector(&self) -> &Matrix<Complex> {
        &self.statevector
    }

    pub fn apply(&mut self, gate: Matrix<Complex>) {
        self.statevector = self.statevector.dot_product(&gate).into();
    }
}
