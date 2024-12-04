from qiskit import ClassicalRegister, QuantumCircuit, QuantumRegister


# L36: test_if_statement()
def create_circuit():
    qr = QuantumRegister(3, 'qr')
    cr = ClassicalRegister(3, 'cr')

    circuit_if_true = QuantumCircuit(qr, cr)
    circuit_if_true.x(qr[0])
    circuit_if_true.x(qr[1])
    circuit_if_true.measure(qr[0], cr[0])
    circuit_if_true.measure(qr[1], cr[1])
    circuit_if_true.x(qr[2]).c_if(cr, 0x3)
    circuit_if_true.measure(qr[0], cr[0])
    circuit_if_true.measure(qr[1], cr[1])
    circuit_if_true.measure(qr[2], cr[2])

    return circuit_if_true
