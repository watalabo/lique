OPENQASM 3.0;
include "stdgates.inc";
bit[1] c;
qubit[1] q2;
if (c == 0) {
  h q2[0];
}
c[0] = measure q2[0];
if (c == 0) {
  h q2[0];
}
c[0] = measure q2[0];
if (c == 0) {
  h q2[0];
}
c[0] = measure q2[0];
