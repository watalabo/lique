# https://github.com/epiqc/PartialCompilation/blob/50d80f56efdf754e40a0b1dd00404788a03fdf3d/qiskit-terra/test/python/test_mapper.py
# -*- coding: utf-8 -*-

# Copyright 2017, IBM.
#
# This source code is licensed under the Apache License, Version 2.0 found in
# the LICENSE.txt file in the root directory of this source tree.

# pylint: disable=missing-docstring,redefined-builtin

from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit


def create_circuit():
    """Check for floating point errors.

    The math library operates over floats and introduces floating point
    errors that should be avoided.
    See: https://github.com/Qiskit/qiskit-terra/issues/111
    """
    qr = QuantumRegister(4)
    cr = ClassicalRegister(4)
    circ = QuantumCircuit(qr, cr)  # ql-constant-classic-bit
    circ.y(qr[0])
    circ.z(qr[2])
    circ.h(qr[2])
    circ.cx(qr[1], qr[0])
    circ.y(qr[2])
    circ.t(qr[2])
    circ.z(qr[2])
    circ.cx(qr[1], qr[2])
    circ.measure(qr[0], cr[0])
    circ.measure(qr[1], cr[1])
    circ.measure(qr[2], cr[2])
    circ.measure(qr[3], cr[3])
    return circ
