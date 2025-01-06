# https://github.com/tsrobinson/quantum/blob/13115571ccbf50b6b5d5f64954b83832ed3155ed/quantum_encoding.py
# Following this course: https://learn.qiskit.org/course/introduction/the-atoms-of-computation
from qiskit import QuantumCircuit


def create_circuit():
    qc2 = QuantumCircuit(3,3) # ql-constant-classic-bit
    qc2.x([0,1])
    qc2.measure([0,1,2], [0,1,2])
    return qc2
