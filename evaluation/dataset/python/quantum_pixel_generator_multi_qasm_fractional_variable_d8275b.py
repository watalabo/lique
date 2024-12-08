# https://github.com/J2304789/Quantum-Superposition-Pixel-Generator/blob/69bcbdf8f644c0d187546a32e4c34068a09991af/Python_Quantum_Superposition_Pixel_Generator/Python_Multi_QASM/Multi_QASM_Fractional/Quantum_Pixel_Generator_Multi_QASM_Fractional_Variable.py
# Import Qiskit and Qiskit.Visualization
from qiskit import QuantumCircuit
from math import pi


def create_circuit():
    # Intializes Quantum Circuit with 2 Qubits and 1 Classical Bit
    qc = QuantumCircuit(2, 1)

    # Sets 1st Qubit into superposition(|+> basis) using controlled x gate and phase shift s gate
    qc.rx(pi / 2, 1)  # Set to |-i>
    qc.s(1)  # Set to |+>

    # Collapses superposition of 1st Qubit and assigns value to corrosponding Classical bit
    qc.measure(1, 0)

    # sets 2nd Qubit into superposition(|+> or |-> basis) based on if Qubits 3-6 were measured as |0> or |1>
    qc.ry(pi / 2, 0)  # Set to |+>
    qc.cz(1, 0)  # Set to |-> if control qubit is |1>,else stays at |+>
    qc.ry(-pi, 0)  # Set to |+> if qubit was at |->,else shifts to |->

    # Creates barrier between gates and measurements for qc.draw() and optimization level
    qc.barrier()

    # Collapses superposition of 2nd Qubit and assigns value to corrosponding Classical bit
    qc.measure(0, 0)
    return qc
