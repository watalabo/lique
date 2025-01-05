# https://github.com/epiqc/PartialCompilation/blob/50d80f56efdf754e40a0b1dd00404788a03fdf3d/qiskit-terra/test/python/transpiler/test_decompose.py
# -*- coding: utf-8 -*-

# Copyright 2018, IBM.
#
# This source code is licensed under the Apache License, Version 2.0 found in
# the LICENSE.txt file in the root directory of this source tree.

"""Test the decompose pass"""

from sympy import pi

from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit


def create_circuit():
    """Test decompose a 1-qubit gates with a conditional.
    """
    qr = QuantumRegister(1, 'qr')
    cr = ClassicalRegister(1, 'cr')
    ref_circuit = QuantumCircuit(qr, cr)
    ref_circuit.u2(0, pi, qr[0]).c_if(cr, 1) # ql-conditional-without-measurement
    ref_circuit.x(qr).c_if(cr, 1) # ql-conditional-without-measurement
    return ref_circuit
