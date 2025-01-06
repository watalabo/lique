OPENQASM 3.0;
include "stdgates.inc";
bit[1] cr;
qubit[1] qr;
if (cr == 1) {
  ry(pi) qr[0];
}
if (cr == 1) {
  x qr[0];
}
