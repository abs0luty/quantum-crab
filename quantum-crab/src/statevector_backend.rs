use num::One;

use crate::{
    backend::Backend,
    classical_register::ClassicalRegister,
    complex::Complex,
    gates::{hadamard, identity},
    matrix::Matrix,
    quantum_circuit::{Instruction, QuantumCircuit},
    quantum_register::QuantumRegister,
};

#[derive(Debug)]
pub struct StatevectorBackend;



impl Backend for StatevectorBackend {
    type Output = Matrix<Complex>;

    fn execute(circuit: QuantumCircuit) -> Matrix<Complex> {
        let mut output = Matrix::new_with_default_elems(2usize.pow(circuit.qubits() as u32), 1);
        output.set(0, 0, Complex::one());

        for instruction in circuit.instructions() {
            match instruction {
                &Instruction::Hadamard(qubit) => {
                    let mut gate = hadamard();
                    let identity = identity();
                    for i in 0..qubit {
                        gate = identity.tensor_product(&gate);
                    }
                    for j in (qubit + 1)..circuit.qubits() {
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
