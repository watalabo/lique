OPENQASM 3.0;
include "stdgates.inc";
bit[3] c9;
qubit[3] q8;
h q8[0];
h q8[1];
c9[0] = measure q8[0];
c9[1] = measure q8[1];
if (c9 == 3) {
  x q8[0];
}
c9[0] = measure q8[0];
c9[1] = measure q8[1];
c9[2] = measure q8[2];
