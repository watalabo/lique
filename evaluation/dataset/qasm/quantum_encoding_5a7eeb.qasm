OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[3] q;
x q[0];
x q[1];
c[0] = measure q[0];
c[1] = measure q[1];
c[2] = measure q[2];
