from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister


def create_circuit():
    i = -1
    qr = QuantumRegister(2)
    cr = ClassicalRegister(8)
    c = QuantumCircuit(qr,cr) # ql-constant-classic-bit
    c.x(qr[1])
    c.measure(qr[0],(i:=i+1))
    c.measure(qr[1],(i:=i+1))
    c.z(qr[0]) # ql-operation-after-measurement
    c.measure(qr[0],(i:=i+1)) # ql-double-measurement
    c.measure(qr[1],(i:=i+1)) # ql-double-measurement
    c.cy(qr[0],qr[1]) # ql-operation-after-measurement
    c.measure(qr[0],(i:=i+1)) # ql-double-measurement
    c.measure(qr[1],(i:=i+1)) # ql-double-measurement
    c.z(qr[0]) # ql-operation-after-measurement
    c.measure(qr[0],(i:=i+1)) # ql-double-measurement
    c.measure(qr[1],(i:=i+1)) # ql-double-measurement
    return c
