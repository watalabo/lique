OPENQASM 3.0;
include "stdgates.inc";
bit[2] c5;
qubit[3] q3;
h q3[1];
cx q3[1], q3[2];
barrier q3[0], q3[1], q3[2];
cx q3[0], q3[1];
h q3[0];
c5[0] = measure q3[0];
c5[1] = measure q3[1];
cx q3[0], q3[2];
cz q3[1], q3[2];
