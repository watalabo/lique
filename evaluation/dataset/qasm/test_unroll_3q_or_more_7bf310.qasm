OPENQASM 3.0;
include "stdgates.inc";
bit[1] cr;
qubit[3] qr;
if (cr == 0) {
  ccx qr[0], qr[1], qr[2];
}
