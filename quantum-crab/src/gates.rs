use crate::{complex::Complex, matrix::Matrix};
use num::One;

type Gate = Matrix<Complex>;

pub fn identity() -> Gate {
    Matrix::identity(2)
}

/// The Hadamard gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Hadamard_transform#Quantum_computing_applications)
/// for more information.
pub fn hadamard() -> Gate {
    matrix_real![[1, 1], [1, -1]] * real![0.7071067811865475]
}

/// The Pauli-X gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-X_gate)
/// for more information.
pub fn pauli_x() -> Gate {
    matrix_real![[0, 1], [1, 0]]
}

/// The Pauli-Y gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-Y_gate)
/// for more information.
pub fn pauli_y() -> Gate {
    matrix![
        [Complex::zero(), -Complex::i()],
        [Complex::i(), Complex::zero()]
    ]
}

/// The Pauli-Z gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-Z_gate)
/// for more information.
pub fn pauli_z() -> Gate {
    matrix_real![[1, 0], [0, -1]]
}

/// A single qubit phase-shift gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Phase_shift_gates)
/// for more information.
pub fn phase_shift(phase: f64) -> Gate {
    matrix![
        [Complex::one(), Complex::zero()],
        [Complex::zero(), Complex::new_from_polar(1f64, phase)]
    ]
}
