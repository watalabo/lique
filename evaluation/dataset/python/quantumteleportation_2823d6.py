# https://github.com/MarcoArmenta/quantum-circuits/blob/134d7de0b45c37b54322d0a4c1fc138e4643ad53/QuantumTeleportation.py
from qiskit import QuantumCircuit


def create_circuit():
    teleport = QuantumCircuit(3, 3)
    teleport.x(0)  # initialize q0=|1> for teleportation
    teleport.h(1)  # entangle qubits q1 and q2
    teleport.cx(1, 2)
    teleport.cx(0, 1)
    teleport.h(0)
    teleport.measure([0, 1], [0, 1])
    teleport.cx(1, 2)
    teleport.cz(0, 2)
    teleport.measure(2, 2)
    return teleport
