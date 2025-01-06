# https://github.com/Melstolit/QisKitTraining/blob/fc42ec0feb9e6302dcf72fbcf3af3eab9668ef8a/Bernaz.py
# Importing standard Qiskit libraries and configuring account
from qiskit import QuantumCircuit


def create_circuit():
    meas = QuantumCircuit(4, 4)
    meas.barrier(range(4))
    meas.measure(range(4),range(4))
    return meas
