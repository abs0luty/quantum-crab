use crate::{
    backend::Backend,
    complex::Complex,
    gates::{hadamard, identity, pauli_x, pauli_y, pauli_z, phase_shift, t},
    matrix::Matrix,
    quantum_circuit::{Instruction, QuantumCircuit},
};
use num::One;

///
#[derive(Debug)]
pub struct StatevectorBackend;

fn execute_single_qubit_instruction(
    instruction: &Instruction,
    circuit: &QuantumCircuit,
    qubit: usize,
    statevector: &mut Matrix<Complex>,
) {
    let mut gate_matrix = match instruction {
        Instruction::PauliX(..) => pauli_x(),
        Instruction::PauliY(..) => pauli_y(),
        Instruction::PauliZ(..) => pauli_z(),
        Instruction::Hadamard(..) => hadamard(),
        Instruction::Phase(.., phase) => phase_shift(*phase),
        Instruction::T(..) => t(),
        _ => unreachable!(),
    };

    let identity = identity();

    for _ in 0..qubit {
        gate_matrix = gate_matrix.tensor_product(&identity);
    }

    for _ in (qubit + 1)..circuit.qubits() {
        gate_matrix = identity.tensor_product(&gate_matrix);
    }

    *statevector = gate_matrix.dot_product(statevector);
}

impl Backend for StatevectorBackend {
    type Output = Matrix<Complex>;

    fn execute(circuit: QuantumCircuit) -> Matrix<Complex> {
        let mut statevector =
            Matrix::new_with_default_elems(2usize.pow(circuit.qubits() as u32), 1);
        statevector.set(0, 0, Complex::one());

        for instruction in circuit.instructions() {
            match instruction {
                &Instruction::Hadamard(qubit)
                | &Instruction::PauliX(qubit)
                | &Instruction::PauliY(qubit)
                | &Instruction::PauliZ(qubit)
                | &Instruction::Phase(qubit, ..)
                | &Instruction::T(qubit) => {
                    execute_single_qubit_instruction(instruction, &circuit, qubit, &mut statevector)
                }
                _ => todo!(),
            }
        }

        statevector
    }
}

#[test]
pub fn test_one_qubit_gates() {
    let mut circuit = QuantumCircuit::new(3);
    circuit.add(Instruction::Hadamard(0));
    circuit.add(Instruction::PauliX(0));
    circuit.add(Instruction::PauliX(1));
    circuit.add(Instruction::T(1));
    circuit.add(Instruction::Hadamard(1));
    circuit.add(Instruction::T(2));
    circuit.add(Instruction::Hadamard(2));
    // let statevector = StatevectorBackend::execute(circuit);
}
