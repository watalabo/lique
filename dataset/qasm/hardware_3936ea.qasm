OPENQASM 3.0;
include "stdgates.inc";
bit[5] c;
qubit[5] q;
h q[2];
cx q[2], q[1];
cx q[2], q[3];
cx q[1], q[0];
cx q[3], q[4];
cx q[0], q[1];
cx q[4], q[3];
barrier q[0], q[1], q[2], q[3], q[4];
x q[2];
cx q[0], q[1];
cx q[4], q[3];
cx q[2], q[1];
cx q[2], q[3];
c[0] = measure q[1];
c[1] = measure q[3];
barrier q[0], q[1], q[2], q[3], q[4];
cx q[0], q[1];
cx q[4], q[3];
cx q[2], q[1];
cx q[2], q[3];
c[2] = measure q[1];
c[3] = measure q[3];
