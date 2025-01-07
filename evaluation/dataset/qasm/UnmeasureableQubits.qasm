OPENQASM 3.0;
include "stdgates.inc";
bit[2] c16;
qubit[3] q19;
h q19[0];
cx q19[0], q19[1];
cx q19[1], q19[2];
c16[0] = measure q19[0];
c16[1] = measure q19[1];
