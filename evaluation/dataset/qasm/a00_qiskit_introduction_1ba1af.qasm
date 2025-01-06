OPENQASM 3.0;
include "stdgates.inc";
bit[2] c0;
qubit[2] q0;
h q0[0];
c0[0] = measure q0[0];
c0[1] = measure q0[1];
