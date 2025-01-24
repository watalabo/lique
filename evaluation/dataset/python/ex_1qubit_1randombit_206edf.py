# https://github.com/JavaFXpert/qiskit-trinkets/blob/6eaa3307181bbdf21e84dc7156b167ca8a9d8086/pqc-qiskit/ex_1qubit_1randombit.py
# ex_1qubit_1randombit.py
# This sample generates a single random bit.

from qiskit import QuantumCircuit, ClassicalRegister, QuantumRegister


def create_circuit():
    qr = QuantumRegister(1)
    cr = ClassicalRegister(1)
    meas_circ = QuantumCircuit(qr, cr) # ql-constant-classic-bit
    meas_circ.barrier(qr)
    meas_circ.measure(qr, cr)
    return meas_circ
