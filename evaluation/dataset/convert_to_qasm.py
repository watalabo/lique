from dataclasses import dataclass
import os
from typing import Literal, Union
from qiskit.qasm3 import dump
import tqdm

import python._12_quantum_key_distribution_checkpoint_4165fd
import python.a00_qiskit_introduction_1ba1af
import python.aatoms_of_computation_ed6fac
import python.ahmed154_73ece9_2
import python.ahmed154_73ece9
import python.balanceada_3_11881b
import python.bernaz_035353
import python.c3x_build_f7ba3d
import python.ch_3_28325b_2
import python.ch_3_28325b
import python.deutsch_jozsa_s_20functions_b84a95
import python.ex_1qubit_1randombit_206edf
import python.example_qiskit_conditional_5ae7f6
import python.gen_algr_20c5be
import python.hadamard_b2fac2
import python.hardware_3936ea_2
import python.hardware_3936ea
import python.hello_b14c35_2
import python.hello_b14c35
import python.herrtestcomptimetiffoli_cf1dac
import python.measurement_postulate_single_qubit_f_fb7a34_2
import python.measurement_postulate_single_qubit_f_fb7a34
import python.one_qubit_fb6117
import python.python_quantum_number_generator_1_6_multi_qasm_76fe84
import python.q02_417981_2
import python.q02_417981
import python.q04_qiskit_installation_and_test_7d0f02
import python.qptt_c5e5a7
import python.quantumdice_bc7351
import python.quantum_encoding_5a7eeb
import python.quantummnist_1f1a21
import python.quantum_pixel_generator_multi_qasm_fractional_variable_d8275b
import python.quantum_teleportation_1881c2_2
import python.quantum_teleportation_1881c2
import python.quantumteleportation_2823d6_2
import python.quantumteleportation_2823d6
import python.qubits_d78ad0
import python.quntum_204_dec_2005_202020_206_2042_20am_16e278
import python.rsa_breaker_4_bit_0f27ff
import python.sandbox_e52934
import python.superpositions_26c0f1
import python.task_0f414b
import python.task_2_4f46b2
import python.teleportation_8f1eaf
import python.teleportation_practice_4ae2ea_2
import python.teleportation_practice_4ae2ea
import python.test_circuit_load_from_qasm_bf30ab
import python.test_circuit_qasm_ae5268
import python.test_circuit_to_dag_5333cb
import python.test_compiler_907f66
import python.test_decompose_78559e
import python.test_mapper_217a32
import python.test_mcx_gate_065857
import python.test_structure_aaba85_2
import python.test_structure_aaba85
import python.test_unroll_3q_or_more_7bf310
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

    files = [
        ("_12_quantum_key_distribution_checkpoint_4165fd", python._12_quantum_key_distribution_checkpoint_4165fd.create_circuit),
        ("a00_qiskit_introduction_1ba1af", python.a00_qiskit_introduction_1ba1af.create_circuit),
        ("aatoms_of_computation_ed6fac", python.aatoms_of_computation_ed6fac.create_circuit),
        ("ahmed154_73ece9_2", python.ahmed154_73ece9_2.create_circuit),
        ("ahmed154_73ece9", python.ahmed154_73ece9.create_circuit),
        ("balanceada_3_11881b", python.balanceada_3_11881b.create_circuit),
        ("bernaz_035353", python.bernaz_035353.create_circuit),
        ("c3x_build_f7ba3d", python.c3x_build_f7ba3d.create_circuit),
        ("ch_3_28325b_2", python.ch_3_28325b_2.create_circuit),
        ("ch_3_28325b", python.ch_3_28325b.create_circuit),
        ("deutsch_jozsa_s_20functions_b84a95", python.deutsch_jozsa_s_20functions_b84a95.create_circuit),
        ("ex_1qubit_1randombit_206edf", python.ex_1qubit_1randombit_206edf.create_circuit),
        ("example_qiskit_conditional_5ae7f6", python.example_qiskit_conditional_5ae7f6.create_circuit),
        ("gen_algr_20c5be", python.gen_algr_20c5be.create_circuit),
        ("hadamard_b2fac2", python.hadamard_b2fac2.create_circuit),
        ("hardware_3936ea_2", python.hardware_3936ea_2.create_circuit),
        ("hardware_3936ea", python.hardware_3936ea.create_circuit),
        ("hello_b14c35_2", python.hello_b14c35_2.create_circuit),
        ("hello_b14c35", python.hello_b14c35.create_circuit),
        ("herrtestcomptimetiffoli_cf1dac", python.herrtestcomptimetiffoli_cf1dac.create_circuit),
        ("measurement_postulate_single_qubit_f_fb7a34_2", python.measurement_postulate_single_qubit_f_fb7a34_2.create_circuit),
        ("measurement_postulate_single_qubit_f_fb7a34", python.measurement_postulate_single_qubit_f_fb7a34.create_circuit),
        ("one_qubit_fb6117", python.one_qubit_fb6117.create_circuit),
        ("python_quantum_number_generator_1_6_multi_qasm_76fe84", python.python_quantum_number_generator_1_6_multi_qasm_76fe84.create_circuit),
        ("q02_417981_2", python.q02_417981_2.create_circuit),
        ("q02_417981", python.q02_417981.create_circuit),
        ("q04_qiskit_installation_and_test_7d0f02", python.q04_qiskit_installation_and_test_7d0f02.create_circuit),
        ("qptt_c5e5a7", python.qptt_c5e5a7.create_circuit),
        ("quantumdice_bc7351", python.quantumdice_bc7351.create_circuit),
        ("quantum_encoding_5a7eeb", python.quantum_encoding_5a7eeb.create_circuit),
        ("quantummnist_1f1a21", python.quantummnist_1f1a21.create_circuit),
        ("quantum_pixel_generator_multi_qasm_fractional_variable_d8275b", python.quantum_pixel_generator_multi_qasm_fractional_variable_d8275b.create_circuit),
        ("quantum_teleportation_1881c2_2", python.quantum_teleportation_1881c2_2.create_circuit),
        ("quantum_teleportation_1881c2", python.quantum_teleportation_1881c2.create_circuit),
        ("quantumteleportation_2823d6_2", python.quantumteleportation_2823d6_2.create_circuit),
        ("quantumteleportation_2823d6", python.quantumteleportation_2823d6.create_circuit),
        ("qubits_d78ad0", python.qubits_d78ad0.create_circuit),
        ("quntum_204_dec_2005_202020_206_2042_20am_16e278", python.quntum_204_dec_2005_202020_206_2042_20am_16e278.create_circuit),
        ("rsa_breaker_4_bit_0f27ff", python.rsa_breaker_4_bit_0f27ff.create_circuit),
        ("sandbox_e52934", python.sandbox_e52934.create_circuit),
        ("superpositions_26c0f1", python.superpositions_26c0f1.create_circuit),
        ("task_0f414b", python.task_0f414b.create_circuit),
        ("task_2_4f46b2", python.task_2_4f46b2.create_circuit),
        ("teleportation_8f1eaf", python.teleportation_8f1eaf.create_circuit),
        ("teleportation_practice_4ae2ea_2", python.teleportation_practice_4ae2ea_2.create_circuit),
        ("teleportation_practice_4ae2ea", python.teleportation_practice_4ae2ea.create_circuit),
        ("test_circuit_load_from_qasm_bf30ab", python.test_circuit_load_from_qasm_bf30ab.create_circuit),
        ("test_circuit_qasm_ae5268", python.test_circuit_qasm_ae5268.create_circuit),
        ("test_circuit_to_dag_5333cb", python.test_circuit_to_dag_5333cb.create_circuit),
        ("test_compiler_907f66", python.test_compiler_907f66.create_circuit),
        ("test_decompose_78559e", python.test_decompose_78559e.create_circuit),
        ("test_mapper_217a32", python.test_mapper_217a32.create_circuit),
        ("test_mcx_gate_065857", python.test_mcx_gate_065857.create_circuit),
        ("test_structure_aaba85_2", python.test_structure_aaba85_2.create_circuit),
        ("test_structure_aaba85", python.test_structure_aaba85.create_circuit),
        ("test_unroll_3q_or_more_7bf310", python.test_unroll_3q_or_more_7bf310.create_circuit),
        ("two_qubit_4cff15", python.two_qubit_4cff15.create_circuit),
    ]

    for case in tqdm.tqdm(files):
        with open(f"{qasm_dir}/{case[0]}.qasm", "w") as qasm_f:
            with open(f"{source_map_dir}/{case[0]}.json", "w") as source_map_f:
                circuit = case[1]()
                dump(circuit, qasm_f, source_map_f)
