from dataclasses import dataclass
import os
from typing import Literal, Union
from qiskit.qasm3 import dump
import tqdm

import python.double_measurement
import python.measurement_postulate_single_qubit_f_fb7a34
import python.qubits_d78ad0
import python.qubits_intro_974f92
import python.rsa_breaker_4_bit_0f27ff
import python.sandbox_e52934
import python.test_circuit_to_dag_5333cb
import python.test_qasm_simulator_edd048
import python.test_structure_aaba85
import python._12_quantum_key_distribution_checkpoint_4165fd
import python.ch_3_28325b
import python.demo3_quantum_teleportation_5b8be6
import python.hardware_3936ea
import python.one_qubit_fb6117
import python.python_quantum_number_generator_1_6_multi_qasm_76fe84
import python.q02_417981
import python.q04_qiskit_installation_and_test_7d0f02
import python.quantum_pixel_generator_multi_qasm_fractional_variable_d8275b
import python.quantum_teleportation_1881c2
import python.quantumdice_bc7351
import python.quantumteleportation_2823d6
import python.quntum_204_dec_2005_202020_206_2042_20am_16e278
import python.task_2_4f46b2
import python.teleportation_8f1eaf
import python.teleportation_practice_4ae2ea
import python.two_qubit_4cff15


@dataclass
class DatasetCase:
    rule_id: str
    label_resolution: Union[Literal["TP"], Literal["FP"]]
    file: str
    factory: callable


if __name__ == "__main__":
    if not os.path.exists("./dataset/qasm"):
        os.makedirs("./dataset/qasm")

    cases = [
        # ql-double-measurement
        DatasetCase("ql-double-measurement", "TP", "measurement_postulate_single_qubit_f_fb7a34", python.measurement_postulate_single_qubit_f_fb7a34.create_circuit),
        DatasetCase("ql-double-measurement", "TP", "qubits_d78ad0", python.qubits_d78ad0.create_circuit),
        DatasetCase("ql-double-measurement", "TP", "qubits_intro_974f92", python.qubits_intro_974f92.create_circuit),
        DatasetCase("ql-double-measurement", "TP", "rsa_breaker_4_bit_0f27ff", python.rsa_breaker_4_bit_0f27ff.create_circuit),
        DatasetCase("ql-double-measurement", "TP", "sandbox_e52934", python.sandbox_e52934.create_circuit),
        DatasetCase("ql-double-measurement", "TP", "test_circuit_to_dag_5333cb", python.test_circuit_to_dag_5333cb.create_circuit),
        DatasetCase("ql-double-measurement", "TP", "test_qasm_simulator_edd048", python.test_qasm_simulator_edd048.create_circuit),
        DatasetCase("ql-double-measurement", "FP", "test_structure_aaba85", python.test_structure_aaba85.create_circuit),
        # ql-operation-after-measurement
        DatasetCase("ql-operation-after-measurement", "TP", "_12_quantum_key_distribution_checkpoint_4165fd", python._12_quantum_key_distribution_checkpoint_4165fd.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "ch_3_28325b", python.ch_3_28325b.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "demo3_quantum_teleportation_5b8be6", python.demo3_quantum_teleportation_5b8be6.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "hardware_3936ea", python.hardware_3936ea.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "one_qubit_fb6117", python.one_qubit_fb6117.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "python_quantum_number_generator_1_6_multi_qasm_76fe84", python.python_quantum_number_generator_1_6_multi_qasm_76fe84.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "q02_417981", python.q02_417981.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "q04_qiskit_installation_and_test_7d0f02", python.q04_qiskit_installation_and_test_7d0f02.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "quantum_pixel_generator_multi_qasm_fractional_variable_d8275b", python.quantum_pixel_generator_multi_qasm_fractional_variable_d8275b.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "quantum_teleportation_1881c2", python.quantum_teleportation_1881c2.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "quantumdice_bc7351", python.quantumdice_bc7351.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "quantumteleportation_2823d6", python.quantumteleportation_2823d6.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "quntum_204_dec_2005_202020_206_2042_20am_16e278", python.quntum_204_dec_2005_202020_206_2042_20am_16e278.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "task_2_4f46b2", python.task_2_4f46b2.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "teleportation_8f1eaf", python.teleportation_8f1eaf.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "teleportation_practice_4ae2ea", python.teleportation_practice_4ae2ea.create_circuit),
        DatasetCase("ql-operation-after-measurement", "TP", "two_qubit_4cff15", python.two_qubit_4cff15.create_circuit),
    ]

    for case in tqdm.tqdm(cases):
        with open(f"./dataset/qasm/{case.file}.qasm", "w") as f:
            circuit = case.factory()
            dump(circuit, f)
