use backend::Backend;
use quantum_circuit::{Instruction, InstructionType, QuantumCircuit};
use simulator::SimulatorBackend;

#[macro_use]
mod complex;
#[macro_use]
mod matrix;
mod ascii_circuit_visualizer;
mod backend;
mod classical_register;
mod gates;
mod ket;
mod quantum_circuit;
mod quantum_register;
mod simulator;

fn main() {
    let mut circuit = QuantumCircuit::new(3);
    circuit.add(Instruction::new(InstructionType::Hadamard, vec![1]));
    circuit.add(Instruction::new(InstructionType::Hadamard, vec![2]));
    let result = SimulatorBackend::execute(circuit);
    dbg!(result);
}
