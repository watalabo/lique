# https://github.com/alihakimtaskiran/QuantumDice/blob/5515461e31f49a17f1f6d030854e0b53316badda/QuantumDice.py
from qiskit import QuantumCircuit


def create_circuit():
    qd=QuantumCircuit(3,3)
    qd.h(0)
    qd.measure(0,0)
    qd.h(0)
    qd.measure(0,1)
    qd.h(0)
    qd.measure(0,2)
    return qd
