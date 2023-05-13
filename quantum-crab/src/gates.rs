use std::f64::consts::PI;

use crate::{complex::Complex, matrix::Matrix};
use num::{One, Zero};

type Gate = Matrix<Complex>;

pub fn identity() -> Gate {
    Matrix::identity(2)
}

/// The Hadamard gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Hadamard_transform#Quantum_computing_applications)
/// for more information.
pub fn hadamard() -> Gate {
    matrix_real![[1, 1], [1, -1]] * Complex::from(1f64 / f64::sqrt(2f64))
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

/// T gate (pi/8 single qubit phase-shift gate).
pub fn t() -> Gate {
    phase_shift(PI / 4f64)
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

/// The two qubit swap gate.
///
/// This swaps the value of the first and second qubit.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Swap_gate)
/// for more information.
pub fn swap() -> Gate {
    matrix_real![[1, 0, 0, 0], [0, 0, 1, 0], [0, 1, 0, 0], [0, 0, 0, 1]]
}

/// A two qubit controlled U gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Controlled_gates)
/// for more information.
///
/// # Panics
///
/// Panics if supplied matrix isn't of size 2x2.
pub fn controlled_u(u: &Gate) -> Gate {
    assert_eq!(u.cols(), 2);
    assert_eq!(u.rows(), 2);

    let mut gate = matrix_real![[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    gate.embed(u, 2, 2);

    gate
}

/// The two qubit controlled-X gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Controlled_gates)
/// for more information.
pub fn controlled_x() -> Gate {
    controlled_u(&pauli_x())
}

/// The two qubit controlled-Y gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Controlled_gates)
/// for more information.
pub fn controlled_y() -> Gate {
    controlled_u(&pauli_y())
}

/// The two qubit controlled-Z gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Controlled_gates)
/// for more information.
pub fn controlled_z() -> Gate {
    controlled_u(&pauli_z())
}

/// The three qubit Toffoli gate.
///
/// If the first two bits are in the state |1> , it applies a Pauli-X on the third bit,
/// else it does nothing.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Toffoli_gate)
/// for more information.
pub fn toffoli() -> Gate {
    let mut gate = Matrix::identity(8);
    gate.embed(&pauli_x(), 6, 6);

    gate
}

/// The three qubit Fredkin gate.
///
/// It performs a controlled swap.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Fredkin_gate)
/// for more information.
pub fn fredkin() -> Gate {
    let mut gate = Matrix::identity(8);
    gate.embed(&pauli_x(), 5, 5);

    gate
}
