OPENQASM 3.0;
include "stdgates.inc";
bit[5] cbits;
qubit[5] qubits;
h qubits[0];
h qubits[3];
h qubits[4];
cbits[0] = measure qubits[0];
cbits[1] = measure qubits[1];
cbits[2] = measure qubits[2];
cbits[3] = measure qubits[3];
cbits[4] = measure qubits[4];
