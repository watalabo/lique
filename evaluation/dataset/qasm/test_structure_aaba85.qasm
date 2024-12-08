OPENQASM 3.0;
include "stdgates.inc";
bit[2] c;
bit[1] cond;
qubit[2] q;
if (cond == 0) {
  c[0] = measure q[0];
}
if (cond == 1) {
  c[0] = measure q[0];
}
if (cond == 1) {
  c[1] = measure q[1];
}
