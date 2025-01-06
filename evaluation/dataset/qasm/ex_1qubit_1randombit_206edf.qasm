OPENQASM 3.0;
include "stdgates.inc";
bit[1] c1;
qubit[1] q3;
barrier q3[0];
c1[0] = measure q3[0];
