from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit


# L7: test_circuit_and_dag
def create_circuit():
    """Check convert to dag and back"""
    qr = QuantumRegister(3)
    cr = ClassicalRegister(3)
    circuit_in = QuantumCircuit(qr, cr)
    circuit_in.h(qr[0])
    circuit_in.h(qr[1])
    circuit_in.measure(qr[0], cr[0])
    circuit_in.measure(qr[1], cr[1])
    circuit_in.x(qr[0]).c_if(cr, 0x3)
    circuit_in.measure(qr[0], cr[0])
    circuit_in.measure(qr[1], cr[1])
    circuit_in.measure(qr[2], cr[2])
    return circuit_in
