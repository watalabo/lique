# https://github.com/aash-gates/Quantum-Computing-Jupyter-Notebook/blob/6ba486f84ca1469127c3ce62c894428c4f16372a/QUNTUM%204_Dec%2005,%202020%206%2042%20AM.ipynb
#!/usr/bin/env python
# coding: utf-8


from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit
from numpy import pi


def create_circuit():
    qreg_q = QuantumRegister(4, 'q')
    creg_c = ClassicalRegister(4, 'c')
    circuit = QuantumCircuit(qreg_q, creg_c)

    circuit.h(qreg_q[0])
    circuit.measure(qreg_q[1], creg_c[1])
    circuit.h(qreg_q[0])
    circuit.x(qreg_q[1])
    circuit.measure(qreg_q[2], creg_c[2])
    circuit.h(qreg_q[0])
    circuit.h(qreg_q[1])
    circuit.measure(qreg_q[3], creg_c[3])
    circuit.crz(pi/2, qreg_q[2], qreg_q[0])
    circuit.reset(qreg_q[3])
    circuit.s(qreg_q[0])
    circuit.cx(qreg_q[1], qreg_q[2])
    circuit.measure(qreg_q[3], creg_c[3])
    circuit.sxdg(qreg_q[0])
    circuit.measure(qreg_q[0], creg_c[0])
    return circuit
