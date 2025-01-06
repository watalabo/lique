# https://github.com/Seanaventure/HighErrorRateRouting/blob/dd4d9d3ea04076d278ccfc859a2baf94c3cc654f/HERRTestCompTimeTiffoli.py
from qiskit import QuantumCircuit


def create_circuit():
    qcHERR = QuantumCircuit(4)  # ql-constant-classic-bit
    qcHERR.x(0)
    qcHERR.x(1)
    qcHERR.ccx(0, 1, 2)
    qcHERR.measure_all()
    return qcHERR
