# https://github.com/epiqc/PartialCompilation/blob/50d80f56efdf754e40a0b1dd00404788a03fdf3d/qiskit-terra/test/python/circuit/test_circuit_qasm.py
# -*- coding: utf-8 -*-

# Copyright 2018, IBM.
#
# This source code is licensed under the Apache License, Version 2.0 found in
# the LICENSE.txt file in the root directory of this source tree.

# pylint: disable=unused-import

"""Test Qiskit's QuantumCircuit class."""
from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit


def create_circuit():
    """Test circuit qasm() method.
    """
    qr1 = QuantumRegister(1, 'qr1')
    qr2 = QuantumRegister(2, 'qr2')
    cr = ClassicalRegister(3, 'cr')
    qc = QuantumCircuit(qr1, qr2, cr)
    qc.s(qr2[1])
    qc.sdg(qr2[1])
    qc.cx(qr1[0], qr2[1])
    qc.barrier(qr2)
    qc.cx(qr2[1], qr1[0])
    qc.h(qr2[1])
    qc.x(qr2[1]).c_if(cr, 0) # ql-conditional-without-measurement
    qc.y(qr1[0]).c_if(cr, 1) # ql-conditional-without-measurement
    qc.z(qr1[0]).c_if(cr, 2) # ql-conditional-without-measurement
    qc.barrier(qr1, qr2)
    qc.measure(qr1[0], cr[0])
    qc.measure(qr2[0], cr[1])
    qc.measure(qr2[1], cr[2])
    return qc
