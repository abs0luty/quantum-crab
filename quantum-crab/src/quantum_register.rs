use std::cell::Cell;

use crate::{classical_register::ClassicalRegister, complex::Complex, matrix::Matrix};

#[derive(Debug)]
pub struct QuantumRegister {
    width: usize,
    state: Matrix<Complex>,
    collapsed: Cell<bool>,
}

impl From<ClassicalRegister> for QuantumRegister {
    fn from(classical: ClassicalRegister) -> Self {
        QuantumRegister {
            width: classical.width(),
            state: classical,
            collapsed: Cell::new(false),
        }
    }
}
