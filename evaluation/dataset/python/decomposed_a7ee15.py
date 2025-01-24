# https://github.com/EmilBahnsenMigration/masters-source/blob/a8f7626e6215afbebf73ca294f89c71eacd93808/gates/decomposed.py
from qiskit import QuantumRegister, QuantumCircuit


def create_circuit():
    # H(N,3) * X(N,1) * X(N,0)
    qr = QuantumRegister(4)
    qc = QuantumCircuit(qr) # ql-unmeasurable-qubits
    qc.x(qr[0])
    qc.x(qr[1])
    qc.h(qr[3])
    return qc
