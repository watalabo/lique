OPENQASM 3.0;
include "stdgates.inc";
bit[2] c;
qubit[2] q2;
c[0] = measure q2[0];
c[1] = measure q2[1];
if (c == 0) {
  h q2[0];
}
c[0] = measure q2[0];
c[1] = measure q2[1];
if (c == 0) {
  h q2[0];
}
c[0] = measure q2[0];
c[1] = measure q2[1];
