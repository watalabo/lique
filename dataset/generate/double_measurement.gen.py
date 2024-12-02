import argparse
import random 


def generate_circuit(n_qubits: int, depth: int, seed: int) -> str:
    code = f"""
from qiskit import QuantumCircuit
circuit = QuantumCircuit({n_qubits}, {n_qubits})
"""
    random.seed(seed)
    for _ in range(depth):
        target_qubit = random.randint(0, n_qubits - 1)
        code += f"circuit.measure({target_qubit}, {target_qubit})\n"
    return code


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate a quantum circuit with measurements.")
    parser.add_argument("n_qubits", type=int, help="Number of qubits in the circuit.")
    parser.add_argument("depth", type=int, help="Number of measurements in the circuit.")
    parser.add_argument("seed", type=int, nargs="?", default=0, help="Random seed.")
    args = parser.parse_args()
    with open("./thesis/dataset/python/double_measurement.py", "w") as f:
        code = generate_circuit(args.n_qubits, args.depth, args.seed)
        f.write(code)
