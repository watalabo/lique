OPENQASM 3.0;
include "stdgates.inc";
bit[16] c2;
qubit[16] q0;
x q0[2];
c2[0] = measure q0[0];
c2[1] = measure q0[1];
c2[2] = measure q0[2];
c2[3] = measure q0[3];
c2[4] = measure q0[4];
c2[5] = measure q0[5];
c2[6] = measure q0[6];
c2[7] = measure q0[7];
c2[8] = measure q0[8];
c2[9] = measure q0[9];
c2[10] = measure q0[10];
c2[11] = measure q0[11];
c2[12] = measure q0[12];
c2[13] = measure q0[13];
c2[14] = measure q0[14];
c2[15] = measure q0[15];
x q0[0];
x q0[1];
x q0[2];
cx q0[2], q0[1];
cx q0[1], q0[2];
cx q0[2], q0[1];
cx q0[1], q0[0];
cx q0[0], q0[1];
cx q0[1], q0[0];
cx q0[3], q0[0];
cx q0[0], q0[1];
cx q0[1], q0[0];
c2[0] = measure q0[0];
c2[1] = measure q0[1];
c2[2] = measure q0[2];
c2[3] = measure q0[3];
c2[4] = measure q0[4];
c2[5] = measure q0[5];
c2[6] = measure q0[6];
c2[7] = measure q0[7];
c2[8] = measure q0[8];
c2[9] = measure q0[9];
c2[10] = measure q0[10];
c2[11] = measure q0[11];
c2[12] = measure q0[12];
c2[13] = measure q0[13];
c2[14] = measure q0[14];
c2[15] = measure q0[15];
x q0[0];
x q0[1];
x q0[2];
cx q0[2], q0[1];
cx q0[1], q0[2];
cx q0[2], q0[1];
cx q0[1], q0[0];
cx q0[0], q0[1];
cx q0[1], q0[0];
cx q0[3], q0[0];
cx q0[0], q0[1];
cx q0[1], q0[0];
