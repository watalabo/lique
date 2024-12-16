from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit


def create_circuit():
    q = QuantumRegister(3)
    c = ClassicalRegister(2)
    circuit = QuantumCircuit(q,c)
    circuit.h(1)
    circuit.cx(1, 2)
    circuit.barrier()
    circuit.cx(0, 1)
    circuit.h(0)
    circuit.measure(0, 0)
    circuit.measure(1, 1)
    circuit.cx(0, 2) # ql-operation-after-measurement
    circuit.cz(1, 2) # ql-operation-after-measurement
    return circuit
