# https://github.com/Ta-SeenJunaid/Quantum-Computing/blob/e80d85d35f85bc04cd748fff0b66a3d72c63b3fa/Quantum%20Cryptography/Task_2.py
from qiskit import QuantumRegister, ClassicalRegister
from qiskit import QuantumCircuit


def create_circuit():
    q = QuantumRegister(5, "q")
    c = ClassicalRegister(4, "c")

    circuit = QuantumCircuit(q, c) # ql-unmeasurable-qubits

    circuit.h(q[0])
    circuit.h(q[1])
    circuit.measure(q[0], c[0])
    circuit.measure(q[1], c[1])
    circuit.ch(q[1], q[0]) # ql-operation-after-measurement
    circuit.barrier()

    circuit.h(q[2])
    circuit.measure(q[2], c[2])
    circuit.ch(q[2], q[0]) # ql-operation-after-measurement

    circuit.measure(q[0], c[3]) # ql-double-measurement
    return circuit
