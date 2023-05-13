use crate::quantum_circuit::QuantumCircuit;

/// Represents backend, that executes quantum circuit and returns
/// any kind of representation of result.
///
/// For example [`crate::statevector_backend::StatevectorBackend`].
pub trait Backend {
    /// Type of output data, that is returned after the circuit is executed.
    type Output;

    /// Executes given quantum circuit and returns the output data
    /// corresponding to different types of backends.
    fn execute(circuit: QuantumCircuit) -> Self::Output;
}
