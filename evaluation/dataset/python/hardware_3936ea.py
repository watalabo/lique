from qiskit import QuantumCircuit


def create_circuit():
    qc_simp = QuantumCircuit(5, 5)
    qc_simp.h(2)
    qc_simp.cx([2, 2, 1, 3, 0, 4], [1, 3, 0, 4, 1, 3])
    qc_simp.barrier([0, 1, 2, 3, 4])
    qc_simp.x(2)
    qc_simp.cx([0, 4, 2, 2], [1, 3, 1, 3])
    qc_simp.measure([1, 3], [0, 1])
    qc_simp.barrier([0, 1, 2, 3, 4])
    qc_simp.cx([0, 4, 2, 2], [1, 3, 1, 3]) # ql-operaton-after-measurement
    qc_simp.measure([1, 3], [2, 3]) # ql-double-measurement
    return qc_simp
