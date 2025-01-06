OPENQASM 3.0;
include "stdgates.inc";
bit[4] c0;
bit[4] c1;
qubit[1] q;
x q[0];
if (c1 == 4) {
  x q[0];
}
