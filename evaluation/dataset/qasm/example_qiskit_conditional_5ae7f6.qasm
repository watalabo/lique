OPENQASM 3.0;
include "stdgates.inc";
bit[1] c0;
bit[1] c1;
bit[1] c2;
qubit[3] q;
h q[0];
if (c0 == 0) {
  h q[1];
}
if (c1 == 1) {
  h q[2];
}
c0[0] = measure q[0];
c1[0] = measure q[1];
c2[0] = measure q[2];
