OPENQASM 3.0;
include "stdgates.inc";
bit[3] c6;
qubit[3] q2;
h q2[0];
h q2[1];
c6[0] = measure q2[0];
c6[1] = measure q2[1];
if (c6 == 3) {
  x q2[0];
}
c6[0] = measure q2[0];
c6[1] = measure q2[1];
c6[2] = measure q2[2];
