# https://github.com/epiqc/PartialCompilation/blob/50d80f56efdf754e40a0b1dd00404788a03fdf3d/qiskit-terra/test/python/transpiler/test_unroll_3q_or_more.py
# -*- coding: utf-8 -*-

# Copyright 2018, IBM.
#
# This source code is licensed under the Apache License, Version 2.0 found in
# the LICENSE.txt file in the root directory of this source tree.

"""Test the Unroll3qOrMore pass"""

from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit


def create_circuit():
    """Test decompose a 3-qubit gate with a conditional.
    """
    qr = QuantumRegister(3, 'qr')
    cr = ClassicalRegister(1, 'cr')
    circuit = QuantumCircuit(qr, cr)
    circuit.ccx(qr[0], qr[1], qr[2]).c_if(cr, 0) # ql-conditional-without-measurement
    return circuit
