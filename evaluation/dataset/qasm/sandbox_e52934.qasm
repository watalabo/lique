OPENQASM 3.0;
include "stdgates.inc";
bit[8] c10;
qubit[2] q13;
x q13[1];
c10[0] = measure q13[0];
c10[1] = measure q13[1];
z q13[0];
c10[2] = measure q13[0];
c10[3] = measure q13[1];
cy q13[0], q13[1];
c10[4] = measure q13[0];
c10[5] = measure q13[1];
z q13[0];
c10[6] = measure q13[0];
c10[7] = measure q13[1];
