use crate::{matrix::Matrix, quantum_circuit::QuantumCircuit};

pub trait Backend {
    fn execute(circuit: QuantumCircuit) -> Matrix<f64>;
}
