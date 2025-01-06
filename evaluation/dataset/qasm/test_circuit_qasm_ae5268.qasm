OPENQASM 3.0;
include "stdgates.inc";
bit[3] cr;
qubit[1] qr1;
qubit[2] qr2;
s qr2[1];
sdg qr2[1];
cx qr1[0], qr2[1];
barrier qr2[0], qr2[1];
cx qr2[1], qr1[0];
h qr2[1];
if (cr == 0) {
  x qr2[1];
}
if (cr == 1) {
  y qr1[0];
}
if (cr == 2) {
  z qr1[0];
}
barrier qr1[0], qr2[0], qr2[1];
cr[0] = measure qr1[0];
cr[1] = measure qr2[0];
cr[2] = measure qr2[1];
