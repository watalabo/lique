from qiskit import QuantumCircuit
from math import pi


def create_circuit():
    qc=QuantumCircuit(6,3)
    qc.rx(pi/2,3)#Set to |-i>
    qc.s(3)#Set to |+>

    ##Collapses superposition of Qubit 3-6 and assigns value to corrosponding Classical bit
    qc.measure(3,3-3)
    qc.rx(pi/2,4)#Set to |-i>
    qc.s(4)#Set to |+>

    ##Collapses superposition of Qubit 3-6 and assigns value to corrosponding Classical bit
    qc.measure(4,4-3)
    qc.rx(pi/2,5)#Set to |-i>
    qc.s(5)#Set to |+>

    ##Collapses superposition of Qubit 3-6 and assigns value to corrosponding Classical bit
    qc.measure(5,5-3)

    qc.barrier()
    qc.ry(pi/2,0)#Set to |+>
    qc.cz(0+3,0)#Set to |-> if control qubit is |1>,else stays at |+>
    qc.ry(-pi,0)#Set to |+> if qubit was at |->,else shifts to |->
    qc.ry(pi/2,1)#Set to |+>
    qc.cz(1+3,1)#Set to |-> if control qubit is |1>,else stays at |+>
    qc.ry(-pi,1)#Set to |+> if qubit was at |->,else shifts to |->
    qc.ry(pi/2,2)#Set to |+>
    qc.cz(2+3,2)#Set to |-> if control qubit is |1>,else stays at |+>
    qc.ry(-pi,2)#Set to |+> if qubit was at |->,else shifts to |->
    qc.measure(0,0)
    qc.measure(1,1)
    qc.measure(2,2)
    return qc
