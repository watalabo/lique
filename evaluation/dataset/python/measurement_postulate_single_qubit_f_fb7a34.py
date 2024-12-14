from math import pi
from qiskit import QuantumCircuit


# L161
def create_circuit():
    circ = QuantumCircuit(1,3)
    circ.measure(0,0)
    circ.ry(pi/2,0) # ql-operaton-after-measurement
    circ.measure(0,1) # ql-double-measurement
    circ.measure(0,2) # ql-double-measurement
    return circ
