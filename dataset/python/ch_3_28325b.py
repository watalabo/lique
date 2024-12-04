from qiskit import QuantumCircuit


# L157
def create_circuit():
    circuit = QuantumCircuit(3,3)
    circuit.h(0)
    circuit.h(1)
    circuit.cx(1,2)
    circuit.cx(0,1)
    circuit.h(0)
    circuit.measure([0, 1], [0, 1])
    circuit.cx(1, 2)
    circuit.cz(0, 2)
    circuit.measure([2], [2])
    return circuit
