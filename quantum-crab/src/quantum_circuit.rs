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
    /// The Identity gate.
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

    /// The Pauli-X gate.
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

    /// The Pauli-Y gate.
    ///
    /// The gate is a single-qubit rotation through pi radians around the y-axis.
    ///
    /// So `PauliY(..) == RotationY(.., PI)`.
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-Y_gate)
    /// for more information.
    PauliY(usize),

    /// The Pauli-Z gate.
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
    /// Hadamard gate matrix is hermitian, so if we apply Hadamard again after we've
    /// created the superposition state, the state will be destroyed.
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Hadamard_transform#Quantum_computing_applications)
    /// for more information.
    Hadamard(usize),

    /// The Phase gate.
    ///
    /// The gate changes the phase in probability amplitude of `|1>`:
    ///
    /// ```txt
    /// |0> --> P(phase) --> |0>
    /// |1> --> P(phase) --> e^(i*phase) |1>
    /// ```
    ///
    /// And it is obvious that:
    ///
    /// ```txt
    /// P(phase) dagger = P(-phase)
    /// ```
    Phase {
        qubit: usize,
        phase: f64,
    },
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
    /// state of the target qubit ([`Instruction::ControlledNot::target`]), if the base state of the control
    /// qubit ([`Instruction::ControlledNot::control`]) is `|1>`:
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
    ControlledNot {
        /// The control qubit.
        ///
        /// See [`Instruction::ControlledNot`] for more information.
        control: usize,

        /// The target qubit.
        ///
        /// See [`Instruction::ControlledNot`] for more information.
        target: usize,
    },

    /// The Controlled-U gate.
    ///
    /// The gate executes gate [`Instruction::ControlledU::gate`] on basis state of the target qubit,
    /// if the basis state of the control qubit is `|1>`:
    /// ```txt
    /// |00> --> CU(0, 1, U = H) --> |00>
    /// |01> --> CU(0, 1, U = H) --> |01>
    /// |10> --> CU(0, 1, U = H) --> 1/sqrt(2)*(|10>+|11>)
    /// |11> --> CU(0, 1, U = H) --> 1/sqrt(2)*(|10>-|11>)
    /// ```
    ControlledU {
        /// The single qubit gate U.
        ///
        /// See [`Instruction::ControlledU`] for more information.
        gate: Box<Instruction>,

        /// The control qubit.
        ///
        /// See [`Instruction::ControlledU`] for more information.
        control: usize,

        /// The target qubit.
        ///
        /// See [`Instruction::ControlledU`] for more information.
        target: usize,
    },

    /// The Swap gate.
    ///
    /// The gate swaps two qubit states:
    ///
    /// ```txt
    /// |a>|b> --> SWAP(0, 1) --> |b>|a>
    /// ```
    ///
    /// And so it is obvious that: `SWAP(a, b) = SWAP(b, a)` and also that
    /// the gate matrix is hermitian.
    SWAP(usize, usize),

    /// The Rotation-X gate.
    /// 
    /// The gate rotates qubit statevector around the X-axis by angle 
    /// [`Instruction::RotationX::phase`].
    RotationX {
        /// The qubit which RX gate is applyed to.
        ///
        /// See [`Instruction::RotationX`] for more information.
        qubit: usize, 

        /// The angle which qubit state is rotate around the X-axis by.
        ///
        /// See [`Instruction::RotationX`] for more information.
        phase: f64
    },

    /// The Rotation-Y gate.
    /// 
    /// The gate rotates qubit statevector around the Y-axis by angle 
    /// [`Instruction::RotationY::phase`].
    RotationY {
        /// The qubit which RX gate is applyed to.
        ///
        /// See [`Instruction::RotationX`] for more information.
        qubit: usize, 

        /// The angle which qubit state is rotate around the X-axis by.
        ///
        /// See [`Instruction::RotationX`] for more information.
        phase: f64
    },
    
    /// The Rotation-Z gate.
    /// 
    /// The gate rotates qubit statevector around the Z-axis by angle 
    /// [`Instruction::RotationZ::phase`].
    RotationZ {
        /// The qubit which RX gate is applyed to.
        ///
        /// See [`Instruction::RotationX`] for more information.
        qubit: usize, 

        /// The angle which qubit state is rotate around the X-axis by.
        ///
        /// See [`Instruction::RotationX`] for more information.
        phase: f64
    },
    

    /// Represents custom gate.
    Custom {
        /// Name of the custom gate.
        name: String, 

        /// Circuit that represents what gate does to input qubits.
        circuit: QuantumCircuit, 

        /// The gates' input qubits.
        input_qubits: Vec<usize>
    },
}
