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
    /// The identity gate instruction.
    ///
    /// The gate just doesn't do anything to the qubit state at all:
    ///
    /// ```txt
    /// |a> --> I --> |a>
    /// ```    
    ///
    /// # Example
    /// ```
    /// let circuit = QuantumCircuit::new(1);
    /// circuit.add(Instruction::Identity(0));
    /// let state_vector = StateVectorBackend::execute(circuit);
    /// assert_eq!(state_vector, matrix_real![[1], [0]]);
    /// ```
    Identity(usize),

    /// The Pauli-X gate instruction.
    ///
    /// The gate flips given qubit probability amplitudes:
    /// ```txt
    /// |0> --> X --> |1>
    /// |1> --> X --> |0>
    /// ```
    /// And so:
    /// ```txt
    /// a|0> + b|1> --> X --> b|0> + a|1>
    /// ```
    ///
    /// It does so by rotation through pi radians around the x-axis. So `PauliX(..) == RotationX(.., PI)`.
    ///
    /// # Example
    /// ```
    /// let circuit = QuantumCircuit::new(1);
    /// circuit.add(Instruction::PauliX(0));
    /// let state_vector = StateVectorBackend::execute(circuit);
    /// assert_eq!(state_vector, matrix_real![[0], [1]]);
    /// ```
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-X_gate)
    /// for more information.
    PauliX(usize),

    /// The Pauli-Y gate instruction.
    ///
    /// The gate is a single-qubit rotation through pi radians around the y-axis.
    ///
    /// So `PauliY(..) == RotationY(.., PI)`.
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-Y_gate)
    /// for more information.
    PauliY(usize),

    /// The Pauli-Z gate instruction.
    ///
    /// The gate is a single-qubit rotation through pi radians around the z-axis.
    ///
    /// So `PauliZ(..) == RotationZ(.., PI)`.
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-Z_gate)
    /// for more information.
    PauliZ(usize),

    /// The Hadamard gate.
    ///
    /// The gate creates a superposition state with equal probabilities out of regular
    /// `|0>` and `|1>`, forming Hadamard basis (`|+>`, `|->`):
    /// ```txt
    /// |0> --> H --> 1/sqrt(2) * (|0> + |1>) or |+> --> H --> |0>
    /// |1> --> H --> 1/sqrt(2) * (|0> - |1>) or |-> --> H --> |1>
    /// ```
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Hadamard_transform#Quantum_computing_applications)
    /// for more information.
    Hadamard(usize),
    Phase(usize, f64),
    T(usize),

    /// The Controlled-NOT gate.
    ///
    /// See [`Instruction::ControlledU`] for more information about controlled gates.
    ///
    /// Controlled-NOT is an analog of XOR operation in quantum computing:
    /// ```txt
    /// |a>|b> --> CNOT(0, 1) --> |a>|b+a>
    /// ```
    ///
    /// In terms of controlled not gates it is defined as the gate that changes the base
    /// state of the target qubit, if the base state of the control qubit is `|1>`:
    ///
    /// ```txt
    /// a|00>+b|01>+c|10>+d|11> --> CNOT(0, 1) --> a|00>+b|01>+d|10>+c|11>
    /// ```
    ///
    /// So the truth table for the gate looks like this:
    ///
    /// | Input A | Input B | Output A | Output B |
    /// |---------|---------|----------|----------|
    /// |    0    |    0    |    0     |    0     |
    /// |    0    |    1    |    0     |    1     |
    /// |    1    |    0    |    1     |    1     |
    /// |    1    |    1    |    1     |    0     |
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Controlled_NOT_gate)
    /// for more information.
    CNOT(usize, usize),
    ControlledU(Box<Instruction>, usize, usize),
    SWAP(usize, usize),
    Toffoli(usize, usize, usize),
    CX(usize, usize),
    RotationX(usize, f64),
    RotationY(usize, f64),
    RotationZ(usize, f64),
    Custom(String, QuantumCircuit, Vec<usize>),
}
