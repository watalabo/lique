# https://github.com/Apress/Quantum-Computing-Solutions/blob/66161505d3f4a1dc0b401912b06afacf796521de/chapter13/Quantum_Teleportation.py


from qiskit import QuantumCircuit as QC


def create_circuit():
    circ = QC(4, 4)

    circ.x(0)
    circ.barrier()

    circ.h(1)
    circ.cx(1, 2)
    circ.cx(0, 1)
    circ.h(0)
    circ.barrier()

    circ.measure(0, 0)
    circ.measure(1, 1)
    circ.barrier()

    circ.cx(1, 3)
    circ.cz(0, 3)
    circ.barrier()

    circ.measure(2, 2)

    return circ
