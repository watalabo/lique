OPENQASM 3.0;
include "stdgates.inc";
bit[1] c;
qubit[1] q1;
if (c == 0) {
  h q1[0];
}
c[0] = measure q1[0];
if (c == 0) {
  h q1[0];
}
c[0] = measure q1[0];
if (c == 0) {
  h q1[0];
}
c[0] = measure q1[0];
