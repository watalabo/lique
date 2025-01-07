from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister


def create_circuit():
    qreg = QuantumRegister(3)
    creg = ClassicalRegister(2)
    qc = QuantumCircuit(qreg, creg) # ql-unmeasurable-qubits
    qc.h(0)
    qc.cx(0, 1)
    qc.cx(1, 2)
    qc.measure([0, 1], [0, 1])
    return qc
