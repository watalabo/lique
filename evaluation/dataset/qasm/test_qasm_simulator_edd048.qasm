OPENQASM 3.0;
include "stdgates.inc";
bit[3] cr;
qubit[3] qr;
x qr[0];
x qr[1];
cr[0] = measure qr[0];
cr[1] = measure qr[1];
if (cr == 3) {
  x qr[2];
}
cr[0] = measure qr[0];
cr[1] = measure qr[1];
cr[2] = measure qr[2];
