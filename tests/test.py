from dummyq import QuantumCircuit, dump


def create_circuit():
    qc = QuantumCircuit(2, 2)
    qc.h(0)
    qc.measure(0, 0)
    qc.h(0)
    return qc


if __name__ == "__main__":
    qc = create_circuit()
    with open("test.qasm", "w") as fq:
        with open("test.qasm.map", "w") as fm:
            dump(qc, fq, fm)
