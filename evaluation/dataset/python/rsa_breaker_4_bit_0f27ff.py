from qiskit import ClassicalRegister, QuantumRegister, QuantumCircuit
from random import randint
import numpy as np


# L107: period(a, N)
def create_circuit():
    available_qubits = 16 
    r=-1   
    N = 4

    qr = QuantumRegister(available_qubits)   
    cr = ClassicalRegister(available_qubits)
    qc = QuantumCircuit(qr,cr)
    x0 = randint(1, N-1) 
    x_binary = np.zeros(available_qubits, dtype=bool)

    for i in range(1, available_qubits + 1):
        bit_state = (N%(2**i)!=0)
        if bit_state:
            N -= 2**(i-1)
        x_binary[available_qubits-i] = bit_state    
    
    for i in range(0,available_qubits):
        if x_binary[available_qubits-i-1]:
            qc.x(qr[i])
    x = x0

    while np.logical_or(x != x0, r <= 0):
        r+=1
        qc.measure(qr, cr) # ql-double-measurement
        qc.x(qr[0]) # ql-operation-after-measurement
        qc.x(qr[1]) # ql-operation-after-measurement
        qc.x(qr[2]) # ql-operation-after-measurement
        qc.cx(qr[2],qr[1]) # ql-operation-after-measurement
        qc.cx(qr[1],qr[2]) # ql-operation-after-measurement
        qc.cx(qr[2],qr[1]) # ql-operation-after-measurement
        qc.cx(qr[1],qr[0]) # ql-operation-after-measurement
        qc.cx(qr[0],qr[1]) # ql-operation-after-measurement
        qc.cx(qr[1],qr[0]) # ql-operation-after-measurement
        qc.cx(qr[3],qr[0]) # ql-operation-after-measurement
        qc.cx(qr[0],qr[1]) # ql-operation-after-measurement
        qc.cx(qr[1],qr[0]) # ql-operation-after-measurement
    return qc
