from qiskit import QuantumCircuit


def create_circuit():
    qc = QuantumCircuit(2) 
    qc.measure_all()
    qc.h(0)  
    qc.measure_all()
    qc.cx(0,1) 
    qc.measure_all()
    return qc
