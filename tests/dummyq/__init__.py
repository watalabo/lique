from abc import ABC, abstractmethod
from dataclasses import asdict, dataclass, field
import inspect
import json
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
class Range:
    line: int
    column_start: int
    column_end: int


@dataclass
class SourceMap:
    source_ranges: list[Range]
    generated_line_byte_offset: list[int]
    generated_file_name: str = field(init=False, default="")


class QuantumCircuit:
    qubits: list[Qubit]
    cbits: list[ClassicalBit]
    gates: list[Instruction]
    source_ranges: list[Range]

    def __init__(self, n_qubits: int, n_cbits: int):
        self.qubits = [Qubit(i) for i in range(n_qubits)]
        self.cbits = [ClassicalBit(i) for i in range(n_cbits)]
        self.gates = []
        self.source_ranges = []

    def h(self, qubit: int):
        caller_frame = inspect.stack()[1]
        source_range = QuantumCircuit._get_caller_range(caller_frame)
        self.source_ranges.append(source_range)
        self.gates.append(H(self.qubits[qubit]))

    def measure(self, qubit, clbit):
        caller_frame = inspect.stack()[1]
        source_range = QuantumCircuit._get_caller_range(caller_frame)
        self.source_ranges.append(source_range)
        self.gates.append(Measurement(self.qubits[qubit], self.cbits[clbit]))

    def to_openqasm(self) -> tuple[str, SourceMap]:
        qasm = "OPENQASM 3.0;\n"
        qasm += 'include "stdgates.inc";\n'
        qasm += f"qubit[{len(self.qubits)}] q;\n"
        qasm += f"bit[{len(self.cbits)}] c;\n"
        qasm_bytes = len(qasm.encode("utf-8"))
        generated_line_byte_offset = []
        for i, gate in enumerate(self.gates):
            instruction = gate.to_openqasm()
            generated_line_byte_offset.append(qasm_bytes)
            qasm_bytes += len(instruction.encode("utf-8"))
            qasm += gate.to_openqasm()
        source_map = SourceMap(self.source_ranges, generated_line_byte_offset)
        return qasm, source_map

    def _get_caller_range(caller_frame: inspect.FrameInfo) -> Range:
        # Decrement line number by 1 because `caller_frame.lineno` is 1-indexed
        line = caller_frame.lineno - 1
        column_start = caller_frame.positions.col_offset
        column_end = caller_frame.positions.end_col_offset
        return Range(line, column_start, column_end)


def dump(circuit: QuantumCircuit, qasm_file: IO[str], source_map_file: IO[str]):
    qasm, source_map = circuit.to_openqasm()
    source_map.file_name = qasm_file.name
    source_map.generated_file_name = qasm_file.name
    qasm_file.write(qasm)
    json.dump(asdict(source_map), source_map_file, indent=4)
