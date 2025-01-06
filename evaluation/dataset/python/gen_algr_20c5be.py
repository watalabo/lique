# https://github.com/Viperr96/Qunatum_things/blob/c52e8e066c7f99b47ec3cec942d58528bf81c6b9/gen/gen_algr.py
# import matplotlib.pyplot as plt
from qiskit import QuantumCircuit, ClassicalRegister, QuantumRegister


def create_circuit():
    q = QuantumRegister(2)
    c = ClassicalRegister(2)
    measureYY = QuantumCircuit(q, c)
    measureYY.sdg(q[0])
    measureYY.sdg(q[1])
    measureYY.h(q[0])
    measureYY.h(q[1])
    measureYY.measure(q[0], c[0])
    measureYY.measure(q[1], c[1])
    return measureYY
