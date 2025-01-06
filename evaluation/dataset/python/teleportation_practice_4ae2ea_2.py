# https://github.com/39xdgy/selflearn_qiskit/blob/874d6aceb8487ec1cd66f5da7c516d54ba541bab/teleportation_practice.py
from qiskit import QuantumCircuit


def create_circuit():
    circuit = QuantumCircuit(4, 4) # ql-oversized-circuit
    # circuit.draw(initial_state=True, output = 'mpl')
    """
    circuit.x(0)
    #circuit.barrier()
    #circuit.draw(output = 'mpl')

    circuit.h(1)
    circuit.cx(1, 2)
    #circuit.draw(output = 'mpl')
    #######################################33
    circuit.cx(0, 1)
    circuit.h(0)
    #circuit.draw(output = 'mpl')

    circuit.barrier()
    circuit.measure([0, 1], [0, 1])
    #circuit.draw(output = 'mpl')

    circuit.barrier()
    circuit.cx(1, 2)
    circuit.cz(0, 2)
    #circuit.draw(output = 'mpl')
    """
    # circuit.barrier()
    circuit.x(0)
    circuit.barrier()
    # circuit.draw(output = 'mpl')

    circuit.h(2)
    circuit.cx(2, 1)

    circuit.cx(0, 2)
    circuit.h(0)
    circuit.barrier()
    circuit.measure([0, 2], [0, 2])

    # circuit.measure([0, 1], [0, 1])
    circuit.measure(1, 1)
    return circuit
