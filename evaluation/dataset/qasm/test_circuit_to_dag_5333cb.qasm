OPENQASM 3.0;
include "stdgates.inc";
bit[3] c4;
qubit[3] q2;
h q2[0];
h q2[1];
c4[0] = measure q2[0];
c4[1] = measure q2[1];
if (c4 == 3) {
  x q2[0];
}
c4[0] = measure q2[0];
c4[1] = measure q2[1];
c4[2] = measure q2[2];
