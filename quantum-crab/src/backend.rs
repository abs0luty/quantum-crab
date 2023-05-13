use crate::{complex::Complex, matrix::Matrix, quantum_circuit::QuantumCircuit};

pub trait Backend {
    type Output;

    fn execute(circuit: QuantumCircuit) -> Self::Output;
}
