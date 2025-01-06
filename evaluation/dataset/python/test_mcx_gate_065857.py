# https://github.com/qclib/qclib/blob/90b4cd886972a7858081bf9cd5709783bfbf6789/test/test_mcx_gate.py
# Copyright 2021 qclib project.

# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at

#     http://www.apache.org/licenses/LICENSE-2.0

# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

""" Test linear mcx with ancilla """

from qiskit import QuantumCircuit


def create_circuit():
    """ Test if linear_mcx is correct """
    mcx_qiskit = QuantumCircuit(6)

    mcx_qiskit.mcx(
        control_qubits=list(range(6 - 2)),
        target_qubit=6 - 2,
        ancilla_qubits=6 - 1,
        mode="recursion"
    )
    return mcx_qiskit
