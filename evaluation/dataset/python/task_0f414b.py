# https://github.com/draymond63/qosf-task-1/blob/cbcda1ace6efc80ad2554ea2b5c9c5b5288444ae/task.py
from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit

"""
Task 1

Design a quantum circuit that considers as input the following vector of integers numbers: 

[1,5,7,10]

returns a quantum state which is a superposition of indices of the target solution, obtaining 
in the output the indices of the inputs where two adjacent bits will always have different values. 
In this case the output should be: |01> + |11>, as the correct indices are 1 and 3.
"""

def create_circuit():
	qreg_q = QuantumRegister(2, 'q')
	creg_c = ClassicalRegister(4, 'c')
	qc = QuantumCircuit(qreg_q, creg_c)

	qc.reset(qreg_q[0])
	qc.reset(qreg_q[1])
	qc.h(qreg_q[1])
	qc.h(qreg_q[0])
	qc.s(qreg_q[1])
	qc.s(qreg_q[0])
	qc.h(qreg_q[1])
	qc.h(qreg_q[0])
	qc.barrier(qreg_q)

	# ! Not happy with c_if using bits
	qc.tdg(qreg_q[0]).c_if(creg_c[0], 1) # ql-conditional-without-measurement
	qc.t  (qreg_q[0]).c_if(creg_c[1], 1) # ql-conditional-without-measurement
	qc.tdg(qreg_q[0]).c_if(creg_c[2], 1) # ql-conditional-without-measurement
	qc.t  (qreg_q[0]).c_if(creg_c[3], 1) # ql-conditional-without-measurement
	qc.tdg(qreg_q[1]).c_if(creg_c[0], 1) # ql-conditional-without-measurement
	qc.tdg(qreg_q[1]).c_if(creg_c[1], 1) # ql-conditional-without-measurement
	qc.t  (qreg_q[1]).c_if(creg_c[2], 1) # ql-conditional-without-measurement
	qc.t  (qreg_q[1]).c_if(creg_c[3], 1) # ql-conditional-without-measurement
	qc.barrier(qreg_q)

	# If only one index is active, move the state to 100% probablity
	qc.tdg(qreg_q[0]).c_if(creg_c, 0b0001) # ql-conditional-without-measurement
	qc.t  (qreg_q[0]).c_if(creg_c, 0b0010) # ql-conditional-without-measurement
	qc.tdg(qreg_q[0]).c_if(creg_c, 0b0100) # ql-conditional-without-measurement
	qc.t  (qreg_q[0]).c_if(creg_c, 0b1000) # ql-conditional-without-measurement
	qc.tdg(qreg_q[1]).c_if(creg_c, 0b0001) # ql-conditional-without-measurement
	qc.tdg(qreg_q[1]).c_if(creg_c, 0b0010) # ql-conditional-without-measurement
	qc.t  (qreg_q[1]).c_if(creg_c, 0b0100) # ql-conditional-without-measurement
	qc.t  (qreg_q[1]).c_if(creg_c, 0b1000) # ql-conditional-without-measurement

	qc.h(qreg_q[0])
	qc.h(qreg_q[1])
	return qc
