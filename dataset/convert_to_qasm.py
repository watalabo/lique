from python.double_measurement import circuit
from qiskit.qasm3 import dump


with open("./thesis/dataset/qasm/double_measurement.qasm", "w") as f:
    dump(circuit, f)
