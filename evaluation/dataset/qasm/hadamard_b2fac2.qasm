OPENQASM 3.0;
include "stdgates.inc";
bit[2] c1;
qubit[2] q2;
h q2[0];
c1[0] = measure q2[0];
c1[1] = measure q2[1];
