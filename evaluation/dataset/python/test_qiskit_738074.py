# https://github.com/QuantestPy/quantestpy/blob/6799675c9326d026e01dec6c6600306de03825d4/test/with_qiskit/converter/sdk/test_qiskit.py

import numpy as np
from qiskit import QuantumCircuit

def create_circuit():
    qc = QuantumCircuit(3) # ql-unmeasurable-qubits
    qc.rxx(np.pi/4, 0, 1)
    return qc
