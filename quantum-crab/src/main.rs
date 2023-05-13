use backend::Backend;
use quantum_circuit::{Instruction, QuantumCircuit};

use crate::statevector_backend::StatevectorBackend;

#[macro_use]
mod complex;
#[macro_use]
mod matrix;
mod ascii_circuit_visualizer;
mod backend;
mod classical_register;
mod gates;
mod quantum_circuit;
mod quantum_register;
mod statevector_backend;

fn main() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.add(Instruction::Hadamard(0));
    circuit.add(Instruction::Hadamard(1));
    let result = StatevectorBackend::execute(circuit);
    println!("Result statevector: {}", result);
}
