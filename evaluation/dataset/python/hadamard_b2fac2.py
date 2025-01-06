# https://github.com/Thoughtscript/x_team_quantum/blob/0eebca323be8b4824a5ceb7c13090757c57d2dd5/quantum-py/hadamard.py
from qiskit import QuantumCircuit, ClassicalRegister, QuantumRegister

def create_circuit():
    # Prepare program
    q = QuantumRegister(2)
    c = ClassicalRegister(2)
    qc = QuantumCircuit(q, c) # ql-constant-classic-bit

    # Put qubit in superposition
    qc.h(q[0])

    # Measure quantum state
    qc.measure(q, c)
    return qc
