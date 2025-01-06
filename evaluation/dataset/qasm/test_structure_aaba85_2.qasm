OPENQASM 3.0;
include "stdgates.inc";
bit[1] cond;
qubit[2] q1;
qubit[2] q2;
if (cond == 0) {
  U(0, 0, 0) q1[0];
}
if (cond == 0) {
  U(0, 0, 0) q1[1];
}
if (cond == 1) {
  cx q1[0], q2[0];
}
if (cond == 1) {
  cx q1[0], q2[1];
}
