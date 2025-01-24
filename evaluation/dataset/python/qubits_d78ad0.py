from qiskit import QuantumCircuit


def create_circuit():
    qc = QuantumCircuit(2) # ql-constant-classic-bit
    qc.measure_all()
    qc.h(0) # ql-operation-after-measurement
    qc.measure_all() # ql-double-measurement
    qc.cx(0,1) # ql-operation-after-measurement
    qc.measure_all() # ql-double-measurement
    return qc
