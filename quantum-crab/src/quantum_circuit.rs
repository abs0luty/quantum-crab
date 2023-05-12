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
pub struct Instruction {
    ty: InstructionType,
    inputs: Vec<usize>,
}

impl Instruction {
    pub fn ty(&self) -> &InstructionType {
        &self.ty
    }

    pub fn inputs(&self) -> &Vec<usize> {
        &self.inputs
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Measure,
    PauliX,
    PauliY,
    PauliZ,
    Hadamard,
    Phase,
    T,
    CNOT,
    SWAP,
    Toffoli,
    CX,
    CCX,
    Custom(String, QuantumCircuit),
}

pub use InstructionType::*;

impl Instruction {
    pub fn new(ty: InstructionType, inputs: Vec<usize>) -> Instruction {
        Instruction { ty, inputs }
    }
}
