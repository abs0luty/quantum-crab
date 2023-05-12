use crate::{classical_register::ClassicalRegister, complex::Complex, ket::Ket, matrix::Matrix};
use std::cell::Cell;

#[derive(Debug)]
pub struct QuantumRegister {
    width: usize,
    state: Ket,
    collapsed: Cell<bool>,
}

impl QuantumRegister {
    #[inline]
    pub fn state(&self) -> &Ket {
        &self.state
    }

    pub fn apply(&mut self, gate: Matrix<Complex>) {
        self.state = gate.dot_product(&self.state.matrix()).into();
    }
}

impl From<ClassicalRegister> for QuantumRegister {
    fn from(classical: ClassicalRegister) -> Self {
        QuantumRegister {
            width: classical.width(),
            state: (&classical).into(),
            collapsed: Cell::new(false),
        }
    }
}
