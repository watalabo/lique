from math import pi
from qiskit import QuantumCircuit


# L161
def create_circuit():
    circ = QuantumCircuit(1,3)
    circ.measure(0,0)
    circ.ry(pi/2,0)
    circ.measure(0,1)
    circ.measure(0,2)
    return circ
