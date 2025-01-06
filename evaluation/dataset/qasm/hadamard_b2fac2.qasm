OPENQASM 3.0;
include "stdgates.inc";
bit[2] c3;
qubit[2] q5;
h q5[0];
c3[0] = measure q5[0];
c3[1] = measure q5[1];
