OPENQASM 3.0;
include "stdgates.inc";
bit[2] c3;
qubit[2] q7;
h q7[0];
c3[0] = measure q7[0];
c3[1] = measure q7[1];
