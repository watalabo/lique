# https://github.com/MarcoArmenta/quantum-circuits/blob/134d7de0b45c37b54322d0a4c1fc138e4643ad53/QuantumTeleportation.py
from qiskit import QuantumCircuit


def create_circuit():
    teleport = QuantumCircuit(4, 4)
    teleport.x(0)
    teleport.h(1)
    teleport.cx(1, 2)
    teleport.cx(0, 1)
    teleport.h(0)
    teleport.measure([0, 1], [0, 1])
    teleport.measure(2, 2)
    return teleport
