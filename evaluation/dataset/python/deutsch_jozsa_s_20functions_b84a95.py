# https://github.com/jcontreras2693/CNYT-Tarea-5/blob/d6a921437714e0bc2f75708d5368002262990aff/Deutsch-Jozsa's%20Functions.py
from qiskit import QuantumCircuit


def create_circuit():
    circuit = QuantumCircuit(5, 5) # ql-constant-classic-bit
    circuit.x(0)
    circuit.barrier()
    circuit.cx(3, 4)
    circuit.barrier()
    circuit.measure([0, 1, 2, 3, 4], [4, 3, 2, 1, 0])
    return circuit
