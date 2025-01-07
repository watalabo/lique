OPENQASM 3.0;
include "stdgates.inc";
bit[3] c12;
qubit[3] q15;
h q15[0];
h q15[1];
c12[0] = measure q15[0];
c12[1] = measure q15[1];
if (c12 == 3) {
  x q15[0];
}
c12[0] = measure q15[0];
c12[1] = measure q15[1];
c12[2] = measure q15[2];
