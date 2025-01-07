OPENQASM 3.0;
include "stdgates.inc";
bit[1] c;
qubit[1] q3;
if (c == 0) {
  h q3[0];
}
c[0] = measure q3[0];
if (c == 0) {
  h q3[0];
}
c[0] = measure q3[0];
if (c == 0) {
  h q3[0];
}
c[0] = measure q3[0];
