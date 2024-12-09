from abc import ABC, abstractmethod
from dataclasses import asdict, dataclass, field
import inspect
from typing import IO


@dataclass
class Qubit:
    index: int


@dataclass
class ClassicalBit:
    index: int


class Instruction(ABC):
    @abstractmethod
    def to_openqasm(self):
        pass


@dataclass
class H(Instruction):
    qubit: Qubit

    def to_openqasm(self):
        return f"h q[{self.qubit.index}];\n"


@dataclass
class Measurement(Instruction):
    qubit: Qubit
    clbit: ClassicalBit

    def to_openqasm(self):
        return f"c[{self.clbit.index}] = measure q[{self.qubit.index}];\n"


@dataclass
class SourceMap:
    source_positions: list[int]
    generated_positions: list[int]
    generated_file_name: str = field(init=False, default="")


@dataclass
class Position:
    lineno: int
    col_offset: int


class QuantumCircuit:
    qubits: list[Qubit]
    cbits: list[ClassicalBit]
    gates: list[Instruction]
    source_lineno: list[int]

    def __init__(self, n_qubits: int, n_cbits: int):
        self.qubits = [Qubit(i) for i in range(n_qubits)]
        self.cbits = [ClassicalBit(i) for i in range(n_cbits)]
        self.gates = []
        self.source_lineno = []

    def h(self, qubit: int):
        caller_frame = inspect.stack()[1]
        lineno = caller_frame.lineno
        print(caller_frame.positions.col_offset, caller_frame.positions.
        self.source_lineno.append(lineno)
        self.gates.append(H(self.qubits[qubit]))

    def measure(self, qubit, clbit):
        lineno = inspect.stack()[1].lineno
        self.source_lineno.append(lineno)
        self.gates.append(Measurement(self.qubits[qubit], self.cbits[clbit]))

    def to_openqasm(self) -> tuple[str, SourceMap]:
        qasm = "OPENQASM 3.0;\n"
        qasm += 'include "stdgates.inc";\n'
        qasm += f"qubit[{len(self.qubits)}] q;\n"
        qasm += f"bit[{len(self.cbits)}] c;\n"
        qasm_lineno = 4
        generated_lineno = []
        for i, gate in enumerate(self.gates):
            instruction = gate.to_openqasm()
            qasm_lineno += instruction.count("\n")
            generated_lineno.append(qasm_lineno)
            qasm += gate.to_openqasm()
        source_map = SourceMap(self.source_lineno, generated_lineno)
        return qasm, source_map


def dump(circuit: QuantumCircuit, qasm_file: IO[str], source_map_file: IO[str]):
    qasm, source_map = circuit.to_openqasm()
    source_map.file_name = qasm_file.name
    source_map.generated_file_name = qasm_file.name
    qasm_file.write(qasm)
    source_map_file.write(str(asdict(source_map)))
