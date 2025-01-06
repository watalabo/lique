# https://github.com/pdebus/IBMQuantumChallenge2020/blob/366552f9f8129f73d5483f8c9d7f9df1afb8d929/src/circuit_parts/superpositions.py
from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit


def create_circuit():
    qubits = QuantumRegister(5, name='qubits')
    cbits = ClassicalRegister(5, name='cbits')

    qc = QuantumCircuit(qubits, cbits) # ql-constant-classic-bit

    qc.h(qubits[0])
    qc.h(qubits[3:])

    qc.measure(qubits, cbits)
    return qc
