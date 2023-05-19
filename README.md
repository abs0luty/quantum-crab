# 🦀 Quantum crab

Quantum crab is a quantum computing crate for Rust programming language.

```rs
use quantum_crab::{
    backend::Backend,
    quantum_circuit::{Instruction, QuantumCircuit},
    statevector_backend::StatevectorBackend,
};

fn main() {
    let mut circuit = QuantumCircuit::new(1);
    circuit.add(Instruction::Hadamard(0));
    let result = StatevectorBackend::execute(circuit);
    println!("{}", result);
}
```

> This shows the circuit that creates superposition state:
>
> ```
> [
>       0.7071067811865475 + 0i
>       0.7071067811865475 + 0i
> ]
> ```
