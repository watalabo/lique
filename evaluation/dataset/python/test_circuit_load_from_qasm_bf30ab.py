# https://github.com/epiqc/PartialCompilation/blob/50d80f56efdf754e40a0b1dd00404788a03fdf3d/qiskit-terra/test/python/circuit/test_circuit_load_from_qasm.py
# -*- coding: utf-8 -*-

# Copyright 2017, IBM.
#
# This source code is licensed under the Apache License, Version 2.0 found in
# the LICENSE.txt file in the root directory of this source tree.


"""Test cases for the circuit qasm_file and qasm_string method."""

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister


def create_circuit():
    qr = QuantumRegister(1, 'q')
    cr0 = ClassicalRegister(4, 'c0')
    cr1 = ClassicalRegister(4, 'c1')
    ref = QuantumCircuit(qr, cr0, cr1)
    ref.x(qr[0])
    ref.x(qr[0]).c_if(cr1, 4) # ql-conditional-without-measurement
    return ref
