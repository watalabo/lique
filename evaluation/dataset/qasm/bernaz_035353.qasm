OPENQASM 3.0;
include "stdgates.inc";
bit[4] c;
qubit[4] q;
barrier q[0], q[1], q[2], q[3];
c[0] = measure q[0];
c[1] = measure q[1];
c[2] = measure q[2];
c[3] = measure q[3];
