# https://github.com/mickahell/quantum_experiments/blob/40c9a193baa5fbe9d8bdf52af63a0606496d92c3/qiskit/hello_quantum/hello.py
#########################################################################################
# Idea from https://medium.com/qiskit/making-a-quantum-computer-smile-cee86a6fc1de
#########################################################################################

import matplotlib.pyplot as plt
from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister


def create_circuit():
    nb_qubits = 16
    # Quantum Circuit
    q = QuantumRegister(nb_qubits)
    c = ClassicalRegister(nb_qubits)

    qc = QuantumCircuit(q, c) # ql-constant-classic-bit

    " ;) = 0011101100101001"
    " 8) = 0011100000101001"

    qc.x(0)
    qc.x(3)
    qc.x(5)
    qc.x(11)
    qc.x(12)
    qc.x(13)

    qc.h(8)
    qc.cx(8, 9)

    qc.measure(range(nb_qubits), range(nb_qubits))
    return qc
