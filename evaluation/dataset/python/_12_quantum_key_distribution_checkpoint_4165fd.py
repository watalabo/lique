from qiskit import QuantumCircuit


# L73
def create_circuit():
    qc = QuantumCircuit(1,1)
    qc.h(0)
    qc.measure(0, 0)
    qc.barrier()
    qc.h(0) # ql-operation-after-measurement
    qc.measure(0,0) # ql-double-measurement
    return qc
