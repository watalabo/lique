# https://github.com/daglarcoban/distributed-quantum-computing/blob/11b971ffd98c9035cc0e4345941ed47ac5d4c121/examples/example_qiskit_conditional.py
"""Example usage of the Quantum Inspire backend with the QisKit SDK.

A simple example that demonstrates how to use the SDK to create
a circuit to demonstrate conditional gate execution.

For documentation on how to use Qiskit we refer to
[https://qiskit.org/](https://qiskit.org/).

Specific to Quantum Inspire is the creation of the QI instance, which is used to set the authentication of the user and
provides a Quantum Inspire backend that is used to execute the circuit.

Copyright 2018-19 QuTech Delft. Licensed under the Apache License, Version 2.0.
"""
from qiskit.circuit import QuantumRegister, ClassicalRegister, QuantumCircuit


def create_circuit():
    q = QuantumRegister(3, "q")
    c0 = ClassicalRegister(1, "c0")
    c1 = ClassicalRegister(1, "c1")
    c2 = ClassicalRegister(1, "c2")
    qc = QuantumCircuit(q, c0, c1, c2, name="conditional")

    qc.h(q[0])
    qc.h(q[1]).c_if(c0, 0)  # ql-conditional-without-measurement
    qc.h(q[2]).c_if(c1, 1)  # ql-conditional-without-measurement

    qc.measure(q[0], c0)
    qc.measure(q[1], c1)
    qc.measure(q[2], c2)

    return qc
