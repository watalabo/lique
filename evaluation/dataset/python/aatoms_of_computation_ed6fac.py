from qiskit import QuantumCircuit


def create_circuit():
    # Create quantum circuit with 3 qubits and 3 classical bits:
    qc = QuantumCircuit(3, 3) # ql-constant-classic-bit
    qc.x([0,1])  # Perform X-gates on qubits 0 & 1
    qc.measure([0,1,2], [0,1,2])
    return qc
