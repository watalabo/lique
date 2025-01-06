from qiskit import QuantumCircuit
from math import pi


def create_circuit():
    qc=QuantumCircuit(6,3) # ql-unmeasurable-qubits
    qc.rx(pi/2,3)
    qc.s(3)

    qc.measure(3,3-3)
    qc.rx(pi/2,4)
    qc.s(4)

    qc.measure(4,4-3)
    qc.rx(pi/2,5)
    qc.s(5)

    qc.measure(5,5-3)

    qc.barrier()
    qc.ry(pi/2,0)
    qc.cz(0+3,0) # ql-operation-after-measurement
    qc.ry(-pi,0)
    qc.ry(pi/2,1)
    qc.cz(1+3,1) # ql-operation-after-measurement
    qc.ry(-pi,1)
    qc.ry(pi/2,2)
    qc.cz(2+3,2) # ql-operation-after-measurement
    qc.ry(-pi,2)
    qc.measure(0,0)
    qc.measure(1,1)
    qc.measure(2,2)
    return qc
