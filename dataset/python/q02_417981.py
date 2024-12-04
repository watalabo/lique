# https://github.com/bzkarimi/Quantum-Computing/blob/7d30a7c19df17107d2e88e6fd188b05ce4b39491/basics/q02.py
#!/usr/bin/env python 

'''
Quantum Teleportation
q0 ---> q2 using q1
'''

from qiskit import *


def create_circuit():
    circuit = QuantumCircuit(3,3)
    circuit.x(0)
    circuit.barrier()
    # q1 and q2 entanglement
    circuit.h(1)
    circuit.cx(1, 2)

    circuit.cx(0, 1)
    circuit.h(0)
    circuit.barrier()
    circuit.measure([0,1], [0,1])
    circuit.barrier()

    circuit.cx(1, 2)
    circuit.cz(0, 2)

    circuit.measure(2,2)
    return circuit
