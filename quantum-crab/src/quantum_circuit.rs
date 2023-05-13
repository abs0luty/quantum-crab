#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCircuit {
    qubits: usize,
    instructions: Vec<Instruction>,
}

impl QuantumCircuit {
    pub fn new(qubits: usize) -> QuantumCircuit {
        QuantumCircuit {
            qubits,
            instructions: Vec::new(),
        }
    }

    pub fn add(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn qubits(&self) -> usize {
        self.qubits
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

pub trait CircuitVisualizer {
    fn visualize_circuit(circuit: QuantumCircuit);
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Measure(usize),
    PauliX(usize),
    PauliY(usize),
    PauliZ(usize),
    Hadamard(usize),
    Phase(usize, f64),
    T(usize),
    CNOT(usize, usize),
    SWAP(usize, usize),
    Toffoli(usize, usize, usize),
    CX(usize, usize),
    Custom(String, QuantumCircuit, Vec<usize>),
}
