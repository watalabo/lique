OPENQASM 3.0;
include "stdgates.inc";
gate sxdg _gate_q_0 {
  s _gate_q_0;
  h _gate_q_0;
  s _gate_q_0;
}
bit[4] c;
qubit[4] q;
h q[0];
c[1] = measure q[1];
h q[0];
x q[1];
c[2] = measure q[2];
h q[0];
h q[1];
c[3] = measure q[3];
crz(pi/2) q[2], q[0];
reset q[3];
s q[0];
cx q[1], q[2];
c[3] = measure q[3];
sxdg q[0];
c[0] = measure q[0];
