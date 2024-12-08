OPENQASM 3.0;
include "stdgates.inc";
bit[5] c;
qubit[5] q;
x q[2];
z q[4];
h q[1];
ry(2*pi/7) q[3];
cx q[1], q[0];
cu(2*pi/7, 0, 0, 0) q[1], q[0];
ccx q[2], q[1], q[0];
barrier q[0], q[1], q[2], q[3], q[4];
c[3] = measure q[0];
c[1] = measure q[1];
c[4] = measure q[2];
c[0] = measure q[3];
c[2] = measure q[4];
barrier q[0], q[1], q[2], q[3], q[4];
if (c == 5) {
  h q[0];
}
if (c == 3) {
  x q[0];
}
if (c == 1) {
  z q[0];
}
h q[0];
x q[1];
h q[2];
x q[3];
h q[4];
cu(6*pi/11, 0, 0, 0) q[1], q[0];
