# https://github.com/Colosu/QPTT/blob/f65c68a7775bf2bcab8730c18302aa2a63bfab29/QPTT.py
#!/usr/bin/env python
# coding: utf-8


# Importing standard Qiskit libraries
from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister

def create_circuit():
    
    input_qubit = QuantumRegister(2, 'input_qubit')
    output_qubit = QuantumRegister(1, 'output_qubit')
    c = ClassicalRegister(1, 'c')
    qc = QuantumCircuit(input_qubit, output_qubit, c) # ql-unmeasurable-qubits
    # Implementation statements
    qc.h(output_qubit)
    qc.cswap(output_qubit, input_qubit[0], input_qubit[1])
    qc.h(output_qubit)
    qc.x(output_qubit)
    qc.measure(output_qubit, c)
    
    return qc
