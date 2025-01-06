OPENQASM 3.0;
include "stdgates.inc";
bit[3] c12;
qubit[3] q12;
h q12[0];
h q12[1];
c12[0] = measure q12[0];
c12[1] = measure q12[1];
if (c12 == 3) {
  x q12[0];
}
c12[0] = measure q12[0];
c12[1] = measure q12[1];
c12[2] = measure q12[2];
