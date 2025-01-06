# https://github.com/ronf6co/ClassiqChallenge/blob/25e571f8262ce0883be2e1b4b5467511d7abbea8/toffoli/c3x_build.py
from qiskit import QuantumCircuit, QuantumRegister


def create_circuit():
    # Init Circuit
    q_i = QuantumRegister(3, 'q_c')
    q_o = QuantumRegister(1, 'q_t')
    qc = QuantumCircuit(q_i, q_o)

    qc.mcx([q_i[0],q_i[1],q_i[2]],q_o[0])
    return qc
