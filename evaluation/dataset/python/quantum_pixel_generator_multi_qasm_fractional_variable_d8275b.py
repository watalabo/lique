# https://github.com/J2304789/Quantum-Superposition-Pixel-Generator/blob/69bcbdf8f644c0d187546a32e4c34068a09991af/Python_Quantum_Superposition_Pixel_Generator/Python_Multi_QASM/Multi_QASM_Fractional/Quantum_Pixel_Generator_Multi_QASM_Fractional_Variable.py
# Import Qiskit and Qiskit.Visualization
from qiskit import QuantumCircuit
from math import pi


def create_circuit():
    
    qc = QuantumCircuit(2, 1) # ql-unmeasurable-qubits

    
    qc.rx(pi / 2, 1)  
    qc.s(1)  

    
    qc.measure(1, 0)

    
    qc.ry(pi / 2, 0)  
    qc.cz(1, 0) # ql-operation-after-measurement  
    qc.ry(-pi, 0)  

    
    qc.barrier()

    
    qc.measure(0, 0)
    return qc
