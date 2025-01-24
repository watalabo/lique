# https://github.com/jakelishman/qiskit-qasm2/blob/d5c145e6ca08f3bdea54822dbdf054c5ce72e02c/tests/test_structure.py
from qiskit.circuit import (
    ClassicalRegister,
    QuantumCircuit,
    QuantumRegister,
)


def create_circuit():
    cond = ClassicalRegister(1, "cond")
    q1 = QuantumRegister(2, "q1")
    q2 = QuantumRegister(2, "q2")
    qc = QuantumCircuit(q1, q2, cond) # ql-unmeasurable-qubits
    qc.u(0, 0, 0, q1[0]).c_if(cond, 0) # ql-conditional-without-measurement
    qc.u(0, 0, 0, q1[1]).c_if(cond, 0) # ql-conditional-without-measurement
    qc.cx(q1[0], q2[0]).c_if(cond, 1) # ql-conditional-without-measurement
    qc.cx(q1[0], q2[1]).c_if(cond, 1) # ql-conditional-without-measurement
    return qc
