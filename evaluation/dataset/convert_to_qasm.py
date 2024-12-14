from dataclasses import asdict, dataclass
import json
import os
from typing import Literal, Union
from qiskit.qasm3 import dump
import tqdm

from common import DatasetCase
import python.measurement_postulate_single_qubit_f_fb7a34
import python.qubits_d78ad0
import python.rsa_breaker_4_bit_0f27ff
import python.sandbox_e52934
import python.test_circuit_to_dag_5333cb
import python.test_qasm_simulator_edd048
import python.test_structure_aaba85
import python._12_quantum_key_distribution_checkpoint_4165fd
import python.ch_3_28325b
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
class LintqResult:
    rule_ids: list[str]
    label_resolution: Union[Literal["TP"], Literal["FP"]]
    file: str
    factory: callable


if __name__ == "__main__":
    dataset_dir = "./evaluation/dataset"
    qasm_dir = f"{dataset_dir}/qasm"
    if not os.path.exists(qasm_dir):
        os.makedirs(qasm_dir)
    source_map_dir = f"{dataset_dir}/source_map"
    if not os.path.exists(source_map_dir):
        os.makedirs(source_map_dir)

    lintq_results = [
        # ql-double-measurement
        LintqResult(["ql-double-measurement", "ql-operation-after-measurement"], "TP", "measurement_postulate_single_qubit_f_fb7a34", python.measurement_postulate_single_qubit_f_fb7a34.create_circuit),
        LintqResult(["ql-double-measurement", "ql-operation-after-measurement"], "TP", "qubits_d78ad0", python.qubits_d78ad0.create_circuit),
        LintqResult(["ql-double-measurement", "ql-operation-after-measurement"], "TP", "rsa_breaker_4_bit_0f27ff", python.rsa_breaker_4_bit_0f27ff.create_circuit),
        LintqResult(["ql-double-measurement", "ql-operation-after-measurement"], "TP", "sandbox_e52934", python.sandbox_e52934.create_circuit),
        LintqResult(["ql-double-measurement"], "TP", "test_circuit_to_dag_5333cb", python.test_circuit_to_dag_5333cb.create_circuit),
        LintqResult(["ql-double-measurement"], "TP", "test_qasm_simulator_edd048", python.test_qasm_simulator_edd048.create_circuit),
        LintqResult(["ql-double-measurement"], "FP", "test_structure_aaba85", python.test_structure_aaba85.create_circuit),
        # ql-operation-after-measurement
        LintqResult(["ql-double-measurement", "ql-operation-after-measurement"], "TP", "_12_quantum_key_distribution_checkpoint_4165fd", python._12_quantum_key_distribution_checkpoint_4165fd.create_circuit),
        LintqResult(["ql-operation-after-measurement"], "TP", "ch_3_28325b", python.ch_3_28325b.create_circuit),
        LintqResult(["ql-double-measurement", "ql-operation-after-measurement"], "TP", "hardware_3936ea", python.hardware_3936ea.create_circuit),
        LintqResult(["ql-operation-after-measurement"], "TP", "one_qubit_fb6117", python.one_qubit_fb6117.create_circuit),
        LintqResult(["ql-operation-after-measurement"], "TP", "python_quantum_number_generator_1_6_multi_qasm_76fe84", python.python_quantum_number_generator_1_6_multi_qasm_76fe84.create_circuit),
        LintqResult(["ql-operation-after-measurement"], "TP", "q02_417981", python.q02_417981.create_circuit),
        LintqResult(["ql-operation-after-measurement"], "TP", "q04_qiskit_installation_and_test_7d0f02", python.q04_qiskit_installation_and_test_7d0f02.create_circuit),
        LintqResult(["ql-operation-after-measurement"], "TP", "quantum_pixel_generator_multi_qasm_fractional_variable_d8275b", python.quantum_pixel_generator_multi_qasm_fractional_variable_d8275b.create_circuit),
        LintqResult(["ql-operation-after-measurement"], "TP", "quantum_teleportation_1881c2", python.quantum_teleportation_1881c2.create_circuit),
        LintqResult(["ql-double-measurement", "ql-operation-after-measurement"], "TP", "quantumdice_bc7351", python.quantumdice_bc7351.create_circuit),
        LintqResult(["ql-operation-after-measurement"], "TP", "quantumteleportation_2823d6", python.quantumteleportation_2823d6.create_circuit),
        LintqResult(["ql-double-measurement", "ql-operation-after-measurement"], "TP", "quntum_204_dec_2005_202020_206_2042_20am_16e278", python.quntum_204_dec_2005_202020_206_2042_20am_16e278.create_circuit),
        LintqResult(["ql-double-measurement", "ql-operation-after-measurement"], "TP", "task_2_4f46b2", python.task_2_4f46b2.create_circuit),
        LintqResult(["ql-double-measurement", "ql-operation-after-measurement"], "TP", "teleportation_8f1eaf", python.teleportation_8f1eaf.create_circuit),
        LintqResult(["ql-operation-after-measurement"], "TP", "teleportation_practice_4ae2ea", python.teleportation_practice_4ae2ea.create_circuit),
        LintqResult(["ql-operation-after-measurement"], "TP", "two_qubit_4cff15", python.two_qubit_4cff15.create_circuit),
    ]

    for case in tqdm.tqdm(lintq_results):
        with open(f"{qasm_dir}/{case.file}.qasm", "w") as qasm_f:
            with open(f"{source_map_dir}/{case.file}.json", "w") as source_map_f:
                circuit = case.factory()
                dump(circuit, qasm_f, source_map_f)

    dataset_cases = []
    for case in lintq_results:
        rule_id = case.rule_ids if case.label_resolution == "TP" else None
        dataset_cases.append(DatasetCase(rule_id, case.file))
    with open(f"{dataset_dir}/dataset.json", "w") as f:
        json.dump([asdict(c) for c in dataset_cases], f, indent=4)
