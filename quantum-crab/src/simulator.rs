use num::One;

use crate::{
    backend::Backend,
    classical_register::ClassicalRegister,
    complex::Complex,
    gates::{hadamard, identity},
    matrix::Matrix,
    quantum_circuit::{InstructionType, QuantumCircuit},
    quantum_register::QuantumRegister,
};

#[derive(Debug)]
pub struct SimulatorBackend;

impl Backend for SimulatorBackend {
    fn execute(circuit: QuantumCircuit) -> Matrix<Complex> {
        let mut output = Matrix::new_with_default_elems(1, 2usize.pow(circuit.qubits() as u32));
        output.set(0, 0, Complex::one());

        for instruction in circuit.instructions() {
            match instruction.ty() {
                &InstructionType::Hadamard => {
                    assert_eq!(instruction.inputs().len(), 1);

                    let qubit_idx = instruction.inputs()[0];

                    let mut gate = hadamard();
                    let identity = identity();
                    for i in 0..qubit_idx {
                        gate = identity.tensor_product(&gate);
                    }
                    for j in 0..qubit_idx {
                        gate = gate.tensor_product(&identity);
                    }

                    output = gate.dot_product(&output);
                }
                _ => todo!(),
            }
        }

        output
    }
}
