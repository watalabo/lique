import os
from qiskit.qasm3 import dump

import python.double_measurement
import python.measurement_postulate_single_qubit_f_fb7a34
import python.qubits_d78ad0
import python.qubits_intro_974f92
import python.rsa_breaker_4_bit_0f27ff
import python.sandbox_e52934
import python.test_circuit_to_dag_5333cb
import python.test_qasm_simulator_edd048
import python.test_structure_aaba85


if __name__ == "__main__":
    if not os.path.exists("./dataset/qasm"):
        os.makedirs("./dataset/qasm")

    file_and_factories = [
        # TP
        ("double_measurement", python.double_measurement.create_circuit),
        ("measurement_postulate_single_qubit_f_fb7a34", python.measurement_postulate_single_qubit_f_fb7a34.create_circuit),
        ("qubits_d78ad0", python.qubits_d78ad0.create_circuit),
        ("qubits_intro_974f92", python.qubits_intro_974f92.create_circuit),
        ("rsa_breaker_4_bit_0f27ff", python.rsa_breaker_4_bit_0f27ff.create_circuit),
        ("sandbox_e52934", python.sandbox_e52934.create_circuit),
        ("test_circuit_to_dag_5333cb", python.test_circuit_to_dag_5333cb.create_circuit),
        ("test_qasm_simulator_edd048", python.test_qasm_simulator_edd048.create_circuit),
        # FP
        ("test_structure_aaba85", python.test_structure_aaba85.create_circuit),
    ]

    for (file, factory) in file_and_factories:
        with open(f"./dataset/qasm/{file}.qasm", "w") as f:
            circuit = factory()
            dump(circuit, f)
