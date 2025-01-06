# https://github.com/niefermar/CuanticaProgramacion/blob/cf066149b4bd769673e83fd774792e9965e5dbc0/test/python/test_compiler.py
# -*- coding: utf-8 -*-

# Copyright 2017, IBM.
#
# This source code is licensed under the Apache License, Version 2.0 found in
# the LICENSE.txt file in the root directory of this source tree.

# pylint: disable=invalid-name

"""Compiler Test."""

import qiskit


def create_circuit():
    """Test compiler doesn't change circuit already matching backend coupling
    """
    q = qiskit.QuantumRegister(16)
    c = qiskit.ClassicalRegister(16)
    qc = qiskit.QuantumCircuit(q, c) # ql-constant-classic-bit
    qc.h(q[1])
    qc.x(q[2])
    qc.x(q[3])
    qc.x(q[4])
    qc.cx(q[1], q[2])
    qc.cx(q[2], q[3])
    qc.cx(q[3], q[4])
    qc.cx(q[3], q[14])
    qc.measure(q, c)
    return qc
