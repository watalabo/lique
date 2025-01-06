from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit


def create_circuit():
    # Create quantum and classical registers with 2 qubits
    qreg = QuantumRegister(2)
    creg = ClassicalRegister(2) 

    # Create a new circuit
    circuit = QuantumCircuit(qreg,creg) # ql-constant-classic-bit

    # Apply H gate to qubit 0
    circuit.h(qreg[0])

    # Measure both qubits
    circuit.measure(qreg,creg)
    return circuit
