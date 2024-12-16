from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit
def apply_secret_unitary(secret_unitary, qubit, quantum_circuit, dagger):
    functionmap = {
                    'x':quantum_circuit.x,
                    'y':quantum_circuit.y,
                    'z':quantum_circuit.z,
                    'h':quantum_circuit.h,
                    't':quantum_circuit.t,
    }
    if dagger:
        functionmap['t'] = quantum_circuit.tdg    
    
    if dagger:
        [functionmap[unitary](qubit) for unitary in secret_unitary]
    else:
        [functionmap[unitary](qubit) for unitary in secret_unitary[::-1]]


def create_circuit():
    secret_0 = 'hxzy'
    secret_1 = 'xtyz'

    # Define register of 3 qubits
    q = QuantumRegister(6)
    c = ClassicalRegister(6)
    circuit = QuantumCircuit(q,c)

    # Do things to Alice's qubit
    apply_secret_unitary(secret_0, 0, circuit, False)
    apply_secret_unitary(secret_1, 1, circuit, False)
    circuit.barrier()

    # Generate entanglement
    circuit.h(2)
    circuit.h(3)
    circuit.cx(2, 4)
    circuit.cx(3, 5)
    circuit.barrier()

    # Perform Bell state measurement
    circuit.cx(1, 3)
    circuit.cx(0, 2)
    circuit.h(1)
    circuit.h(0)
    circuit.measure([0, 1, 2, 3], [0, 1, 2, 3])

    # Operate on Bob's qubit given result
    circuit.cx(3, 4) # ql-operation-after-measurement
    circuit.cz(0, 4) # ql-operation-after-measurement
    circuit.cx(2, 5) # ql-operation-after-measurement
    circuit.cz(1, 5) # ql-operation-after-measurement
    circuit.barrier()

    # Do the reverse things on Bob's qubit
    apply_secret_unitary(secret_0, 4, circuit, True)
    apply_secret_unitary(secret_1, 5, circuit, True)

    # Measure Bob's qubit
    circuit.measure([4, 5], [4, 5])
    return circuit
