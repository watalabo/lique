from qiskit import ClassicalRegister, QuantumCircuit, QuantumRegister


def create_circuit():
    cond = ClassicalRegister(1, "cond")
    qc = QuantumCircuit(QuantumRegister(2, "q"), ClassicalRegister(2, "c"), cond)
    qc.measure(0, 0).c_if(cond, 0) # ql-conditional-without-measurement
    qc.measure(0, 0).c_if(cond, 1) # ql-conditional-without-measurement
    qc.measure(1, 1).c_if(cond, 1) # ql-conditional-without-measurement
    return qc
