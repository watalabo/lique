# Lique
Lique is a fast and framework agnostic linter for quantum programs while utilizing OpenQASM 3.0.
It can lint quantum programs using various frameworks with exported OpenQASM 3.0 code and source maps.

## Installation
Install a binary from [Releases](https://github.com/watalabo/lique/releases).
Or you can build from source code with cargo.
```sh
cargo install lique
```

## Usage
We adapt to Qiskit for now.
Use modified Qiskit to export OpenQASM 3.0 code and source maps.
```sh
pip install git+https://github.com/watalabo/qiskit
```

Write a quantum program with Qiskit and export OpenQASM 3.0 code and source maps.
```python
from qiskit import QuantumCircuit
from qiskit.qasm3 import dump

def create_circuit():
    qc = QuantumCircuit(2)
    qc.h(0)
    qc.cx(0, 1)
    return qc

if __name__ == "__main__":
    c = create_circuit()
    with open("circuit.qasm", "w") as f_qasm:
        with open("circuit.qasm.map.json", "w") as f_map:
            dump(c, f_qasm, f_map)
```

Then lint the quantum program with Lique.
```sh
lique circuit.qasm --source-file circuit.py --source-map circuit.qasm.map.json
```
