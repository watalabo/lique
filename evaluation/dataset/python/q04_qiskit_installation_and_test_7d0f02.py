# https://github.com/Vishal-Mandal/bronze-qiskit-master/blob/1ff2c0f308c22934299fcb1f44a0cd92afd5a7a4/quantum-with-qiskit/Q04_Qiskit_installation_and_test.ipynb
#!/usr/bin/env python
# coding: utf-8

# <a href="https://qworld.net" target="_blank" align="left"><img src="../qworld/images/header.jpg"  align="left"></a>
# $ \newcommand{\bra}[1]{\langle #1|} $
# $ \newcommand{\ket}[1]{|#1\rangle} $
# $ \newcommand{\braket}[2]{\langle #1|#2\rangle} $
# $ \newcommand{\dot}[2]{ #1 \cdot #2} $
# $ \newcommand{\biginner}[2]{\left\langle #1,#2\right\rangle} $
# $ \newcommand{\mymatrix}[2]{\left( \begin{array}{#1} #2\end{array} \right)} $
# $ \newcommand{\myvector}[1]{\mymatrix{c}{#1}} $
# $ \newcommand{\myrvector}[1]{\mymatrix{r}{#1}} $
# $ \newcommand{\mypar}[1]{\left( #1 \right)} $
# $ \newcommand{\mybigpar}[1]{ \Big( #1 \Big)} $
# $ \newcommand{\sqrttwo}{\frac{1}{\sqrt{2}}} $
# $ \newcommand{\dsqrttwo}{\dfrac{1}{\sqrt{2}}} $
# $ \newcommand{\onehalf}{\frac{1}{2}} $
# $ \newcommand{\donehalf}{\dfrac{1}{2}} $
# $ \newcommand{\hadamard}{ \mymatrix{rr}{ \sqrttwo & \sqrttwo \\ \sqrttwo & -\sqrttwo }} $
# $ \newcommand{\vzero}{\myvector{1\\0}} $
# $ \newcommand{\vone}{\myvector{0\\1}} $
# $ \newcommand{\stateplus}{\myvector{ \sqrttwo \\  \sqrttwo } } $
# $ \newcommand{\stateminus}{ \myrvector{ \sqrttwo \\ -\sqrttwo } } $
# $ \newcommand{\myarray}[2]{ \begin{array}{#1}#2\end{array}} $
# $ \newcommand{\X}{ \mymatrix{cc}{0 & 1 \\ 1 & 0}  } $
# $ \newcommand{\I}{ \mymatrix{rr}{1 & 0 \\ 0 & 1}  } $
# $ \newcommand{\Z}{ \mymatrix{rr}{1 & 0 \\ 0 & -1}  } $
# $ \newcommand{\Htwo}{ \mymatrix{rrrr}{ \frac{1}{2} & \frac{1}{2} & \frac{1}{2} & \frac{1}{2} \\ \frac{1}{2} & -\frac{1}{2} & \frac{1}{2} & -\frac{1}{2} \\ \frac{1}{2} & \frac{1}{2} & -\frac{1}{2} & -\frac{1}{2} \\ \frac{1}{2} & -\frac{1}{2} & -\frac{1}{2} & \frac{1}{2} } } $
# $ \newcommand{\CNOT}{ \mymatrix{cccc}{1 & 0 & 0 & 0 \\ 0 & 1 & 0 & 0 \\ 0 & 0 & 0 & 1 \\ 0 & 0 & 1 & 0} } $
# $ \newcommand{\norm}[1]{ \left\lVert #1 \right\rVert } $
# $ \newcommand{\pstate}[1]{ \lceil \mspace{-1mu} #1 \mspace{-1.5mu} \rfloor } $
# $ \newcommand{\greenbit}[1] {\mathbf{{\color{green}#1}}} $
# $ \newcommand{\bluebit}[1] {\mathbf{{\color{blue}#1}}} $
# $ \newcommand{\redbit}[1] {\mathbf{{\color{red}#1}}} $
# $ \newcommand{\brownbit}[1] {\mathbf{{\color{brown}#1}}} $
# $ \newcommand{\blackbit}[1] {\mathbf{{\color{black}#1}}} $

# <font style="font-size:28px;" align="left"><b> Qiskit installation and test </b></font>
# <br>
# _prepared by Abuzer Yakaryilmaz_
# <br><br>

# - [Check your system](#check)
# - [Install qiskit](#install)
# - [Tips](#tips)
# - [Execute an example quantum program](#test)
# - [Complete test](#complete)

# <hr id="check">
#
# ### Check your system
#
# Check your system, if Qiskit has already been installed:


# import the objects from qiskit
from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit
from math import pi


def create_circuit():
    q = QuantumRegister(5, "q")  
    c = ClassicalRegister(5, "c")  
    qc = QuantumCircuit(
        q, c
    )  

    qc.x(q[2])  
    qc.z(q[4])  
    qc.h(q[1])  

    a = pi / 7
    qc.ry(
        2 * a, q[3]
    )  

    qc.cx(
        q[1], q[0]
    )  

    qc.cu(
        2 * a, 0, 0, 0, q[1], q[0]
    )  
    

    qc.ccx(
        q[2], q[1], q[0]
    )  

    qc.barrier()

    qc.measure(q[0], c[3])  
    qc.measure(q[1], c[1])  
    qc.measure(q[2], c[4])  
    qc.measure(q[3], c[0])  
    qc.measure(q[4], c[2])  

    qc.barrier()

    qc.h(q[0]).c_if(c, 5) # ql-operation-after-measurement
    qc.x(q[0]).c_if(c, 3) # ql-operation-after-measurement
    qc.z(q[0]).c_if(c, 1) # ql-operation-after-measurement
    if 0 % 2 == 0:
        qc.h(q[0]) # ql-operation-after-measurement
    else:
        qc.x(q[0])
    if 1 % 2 == 0:
        qc.h(q[1])
    else:
        qc.x(q[1]) # ql-operation-after-measurement
    if 2 % 2 == 0:
        qc.h(q[2]) # ql-operation-after-measurement
    else:
        qc.x(q[2])
    if 3 % 2 == 0:
        qc.h(q[3])
    else:
        qc.x(q[3]) # ql-operation-after-measurement
    if 4 % 2 == 0:
        qc.h(q[4]) # ql-operation-after-measurement
    else:
        qc.x(q[4])
    b = 3 * pi / 11
    qc.cu(2 * b, 0, 0, 0, q[1], q[0]) # ql-operation-after-measurement
    return qc
