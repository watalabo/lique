OPENQASM 3.0;
include "stdgates.inc";
bit[4] c;
qubit[2] q;
reset q[0];
reset q[1];
h q[1];
h q[0];
s q[1];
s q[0];
h q[1];
h q[0];
barrier q[0], q[1];
if (c[0]) {
  tdg q[0];
}
if (c[1]) {
  t q[0];
}
if (c[2]) {
  tdg q[0];
}
if (c[3]) {
  t q[0];
}
if (c[0]) {
  tdg q[1];
}
if (c[1]) {
  tdg q[1];
}
if (c[2]) {
  t q[1];
}
if (c[3]) {
  t q[1];
}
barrier q[0], q[1];
if (c == 1) {
  tdg q[0];
}
if (c == 2) {
  t q[0];
}
if (c == 4) {
  tdg q[0];
}
if (c == 8) {
  t q[0];
}
if (c == 1) {
  tdg q[1];
}
if (c == 2) {
  tdg q[1];
}
if (c == 4) {
  t q[1];
}
if (c == 8) {
  t q[1];
}
h q[0];
h q[1];
