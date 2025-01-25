# https://github.com/ahmedkfu2020/-/blob/a205805a9dfaef2f8cb2ff0645c597b1b119747c/ahmed154.py
# import all necessary objects and methods for quantum circuits
from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit

def create_circuit():
    # define a quantum register with a single qubit
    q = QuantumRegister(2)
    # define a classical register with a single bit
    c = ClassicalRegister(2,"c")
    # define a quantum circuit
    qc = QuantumCircuit(q,c) # ql-constant-classic-bit
    qc.measure(q,c)
    qc.h(q[0]).c_if(c,0) # ql-operation-after-measurement
    qc.measure(q,c) # ql-double-measurement
    qc.h(q[0]).c_if(c,0) # ql-operation-after-measurement
    qc.measure(q,c) # ql-double-measurement
    return qc
