# https://github.com/Nat15005/DeutschYDeutsch-Jozsa/blob/07c15bcc325bfc04979230769f4075e1124fb0b9/Funciones_DEUTSCH-JOZSA/Balanceada_3.py
from qiskit import QuantumCircuit


def create_circuit():
    circuit = QuantumCircuit(5, 5) # ql-constant-classic-bit
    circuit.x(3)
    circuit.cx(1, 4)
    circuit.measure([0,1,2,3,4], [4,3,2,1,0])
    return circuit
