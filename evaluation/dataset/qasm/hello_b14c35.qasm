OPENQASM 3.0;
include "stdgates.inc";
bit[16] c2;
qubit[16] q3;
x q3[0];
x q3[3];
x q3[5];
x q3[11];
x q3[12];
x q3[13];
h q3[8];
cx q3[8], q3[9];
c2[0] = measure q3[0];
c2[1] = measure q3[1];
c2[2] = measure q3[2];
c2[3] = measure q3[3];
c2[4] = measure q3[4];
c2[5] = measure q3[5];
c2[6] = measure q3[6];
c2[7] = measure q3[7];
c2[8] = measure q3[8];
c2[9] = measure q3[9];
c2[10] = measure q3[10];
c2[11] = measure q3[11];
c2[12] = measure q3[12];
c2[13] = measure q3[13];
c2[14] = measure q3[14];
c2[15] = measure q3[15];
