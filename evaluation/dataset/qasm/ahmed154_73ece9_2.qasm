OPENQASM 3.0;
include "stdgates.inc";
bit[2] c;
qubit[2] q1;
c[0] = measure q1[0];
c[1] = measure q1[1];
if (c == 0) {
  h q1[0];
}
c[0] = measure q1[0];
c[1] = measure q1[1];
if (c == 0) {
  h q1[0];
}
c[0] = measure q1[0];
c[1] = measure q1[1];
