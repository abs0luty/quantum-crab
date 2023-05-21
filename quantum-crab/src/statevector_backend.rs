use crate::{
    backend::Backend,
    complex::Complex,
    matrix::Matrix,
    quantum_circuit::{Instruction, QuantumCircuit},
};
use num::{One, Zero};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct StateVectorBackend;

/// Executes single qubit gate `instruction` and applies it to the `statevector`.
fn execute_single_qubit_gate(
    instruction: &Instruction,
    circuit: &QuantumCircuit,
    qubit: usize,
    statevector: &mut Matrix<Complex>,
) {
    let mut gate_matrix = match instruction {
        // If it is identity gate, then we don't do anything with
        // the statevector
        Instruction::Identity(..) => return,
        Instruction::PauliX(..) => matrix_real![[0, 1], [1, 0]],
        Instruction::PauliY(..) => matrix![
            [Complex::zero(), -Complex::i()],
            [Complex::i(), Complex::zero()]
        ],
        Instruction::PauliZ(..) => matrix_real![[1, 0], [0, -1]],
        Instruction::RotationX { phase, .. } => {
            let phase_half = phase / 2f64;
            matrix![
                [
                    Complex::new(phase_half.cos(), 0),
                    Complex::new(0, -phase_half.sin())
                ],
                [
                    Complex::new(0, -phase_half.sin()),
                    Complex::new(phase_half.cos(), 0)
                ]
            ]
        }
        Instruction::RotationY { phase, .. } => {
            let phase_half = phase / 2f64;
            matrix![
                [
                    Complex::new(phase_half.cos(), 0),
                    Complex::new(0, -phase_half.sin())
                ],
                [
                    Complex::new(0, phase_half.sin()),
                    Complex::new(phase_half.cos(), 0)
                ]
            ]
        }
        Instruction::RotationZ { phase, .. } => {
            let phase_half = phase / 2f64;
            matrix![
                [Complex::new_from_polar(1, -phase_half), Complex::zero()],
                [Complex::zero(), Complex::new_from_polar(1, phase_half)]
            ]
        }
        Instruction::Hadamard(..) => {
            matrix_real![[1, 1], [1, -1]] * Complex::new(1f64 / 2f64.sqrt(), 0)
        }
        Instruction::Phase { phase, .. } => matrix![
            [Complex::one(), Complex::zero()],
            [Complex::zero(), Complex::new_from_polar(1, *phase)]
        ],
        Instruction::PhaseDagger { phase, .. } => matrix![
            [Complex::one(), Complex::zero()],
            [Complex::zero(), Complex::new_from_polar(1, -phase)]
        ],
        Instruction::T(..) => matrix![
            [Complex::one(), Complex::zero()],
            [Complex::zero(), Complex::new_from_polar(1, PI / 4f64)]
        ],
        Instruction::TDagger(..) => matrix![
            [Complex::one(), Complex::zero()],
            [Complex::zero(), Complex::new_from_polar(1, -PI / 4f64)]
        ],
        Instruction::S(..) => matrix![
            [Complex::one(), Complex::zero()],
            [Complex::zero(), Complex::new_from_polar(1, PI / 4f64)]
        ],
        Instruction::SDagger(..) => matrix![
            [Complex::one(), Complex::zero()],
            [Complex::zero(), Complex::new_from_polar(1, PI / 4f64)]
        ],
        _ => unreachable!(),
    };

    if qubit != 0 {
        gate_matrix = gate_matrix.tensor_product(&Matrix::identity(2_usize.pow(qubit as u32)));
    }

    for _ in (qubit + 1)..circuit.qubits() {
        gate_matrix = Matrix::identity(2).tensor_product(&gate_matrix);
    }

    *statevector = gate_matrix.dot_product(statevector);
}

impl Backend for StateVectorBackend {
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
                | &Instruction::Phase { qubit, .. }
                | &Instruction::PhaseDagger { qubit, .. }
                | &Instruction::T(qubit)
                | &Instruction::TDagger(qubit)
                | &Instruction::S(qubit)
                | &Instruction::SDagger(qubit)
                | &Instruction::Identity(qubit)
                | &Instruction::RotationX { qubit, .. }
                | &Instruction::RotationY { qubit, .. }
                | &Instruction::RotationZ { qubit, .. } => {
                    execute_single_qubit_gate(instruction, &circuit, qubit, &mut statevector)
                }
                _ => todo!(),
            }
        }

        statevector
    }
}
