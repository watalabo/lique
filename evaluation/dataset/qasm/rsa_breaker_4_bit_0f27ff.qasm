OPENQASM 3.0;
include "stdgates.inc";
bit[16] c9;
qubit[16] q9;
x q9[2];
c9[0] = measure q9[0];
c9[1] = measure q9[1];
c9[2] = measure q9[2];
c9[3] = measure q9[3];
c9[4] = measure q9[4];
c9[5] = measure q9[5];
c9[6] = measure q9[6];
c9[7] = measure q9[7];
c9[8] = measure q9[8];
c9[9] = measure q9[9];
c9[10] = measure q9[10];
c9[11] = measure q9[11];
c9[12] = measure q9[12];
c9[13] = measure q9[13];
c9[14] = measure q9[14];
c9[15] = measure q9[15];
x q9[0];
x q9[1];
x q9[2];
cx q9[2], q9[1];
cx q9[1], q9[2];
cx q9[2], q9[1];
cx q9[1], q9[0];
cx q9[0], q9[1];
cx q9[1], q9[0];
cx q9[3], q9[0];
cx q9[0], q9[1];
cx q9[1], q9[0];
c9[0] = measure q9[0];
c9[1] = measure q9[1];
c9[2] = measure q9[2];
c9[3] = measure q9[3];
c9[4] = measure q9[4];
c9[5] = measure q9[5];
c9[6] = measure q9[6];
c9[7] = measure q9[7];
c9[8] = measure q9[8];
c9[9] = measure q9[9];
c9[10] = measure q9[10];
c9[11] = measure q9[11];
c9[12] = measure q9[12];
c9[13] = measure q9[13];
c9[14] = measure q9[14];
c9[15] = measure q9[15];
x q9[0];
x q9[1];
x q9[2];
cx q9[2], q9[1];
cx q9[1], q9[2];
cx q9[2], q9[1];
cx q9[1], q9[0];
cx q9[0], q9[1];
cx q9[1], q9[0];
cx q9[3], q9[0];
cx q9[0], q9[1];
cx q9[1], q9[0];
