OPENQASM 3.0;
include "stdgates.inc";
bit[1] c1;
qubit[1] q5;
barrier q5[0];
c1[0] = measure q5[0];
