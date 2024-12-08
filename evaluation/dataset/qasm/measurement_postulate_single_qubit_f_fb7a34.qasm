OPENQASM 3.0;
include "stdgates.inc";
bit[3] c;
qubit[1] q;
c[0] = measure q[0];
ry(pi/2) q[0];
c[1] = measure q[0];
c[2] = measure q[0];
