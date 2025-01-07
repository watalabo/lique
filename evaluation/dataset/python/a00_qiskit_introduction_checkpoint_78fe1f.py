# https://github.com/gkorpal/QSilver/blob/744721523e2c5bfdb86afd82517d4f7201bf2bec/silver/.ipynb_checkpoints/A00_Qiskit_Introduction-checkpoint.ipynb
#!/usr/bin/env python
# coding: utf-8

# <table  align="left" width="100%"> <tr>
#         <td  style="background-color:#ffffff;"><a href="https://qsoftware.lu.lv/index.php/qworld/" target="_blank"><img src="..\images\qworld.jpg" width="35%" align="left"></a></td>
#         <td  align="right" style="background-color:#ffffff;vertical-align:bottom;horizontal-align:right">
#             prepared by Özlem Salehi (<a href="http://qworld.lu.lv/index.php/qturkey/" target="_blank">QTurkey</a>)
#         </td>        
# </tr></table>

# <table width="100%"><tr><td style="color:#bbbbbb;background-color:#ffffff;font-size:11px;font-style:italic;text-align:right;">This cell contains some macros. If there is a problem with displaying mathematical formulas, please run this cell to load these macros. </td></tr></table>
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
# $ \newcommand{\Z}{ \mymatrix{rr}{1 & 0 \\ 0 & -1}  } $
# $ \newcommand{\Htwo}{ \mymatrix{rrrr}{ \frac{1}{2} & \frac{1}{2} & \frac{1}{2} & \frac{1}{2} \\ \frac{1}{2} & -\frac{1}{2} & \frac{1}{2} & -\frac{1}{2} \\ \frac{1}{2} & \frac{1}{2} & -\frac{1}{2} & -\frac{1}{2} \\ \frac{1}{2} & -\frac{1}{2} & -\frac{1}{2} & \frac{1}{2} } } $
# $ \newcommand{\CNOT}{ \mymatrix{cccc}{1 & 0 & 0 & 0 \\ 0 & 1 & 0 & 0 \\ 0 & 0 & 0 & 1 \\ 0 & 0 & 1 & 0} } $
# $ \newcommand{\norm}[1]{ \left\lVert #1 \right\rVert } $
# $ \newcommand{\pstate}[1]{ \lceil \mspace{-1mu} #1 \mspace{-1.5mu} \rfloor } $

# <h1> Introduction to Qiskit </h1>
# 
# <hr>

# You can import Qiskit using the following command:

# In[ ]:




# <hr>
# 
# <h2>Creating quantum and classical registers</h2>

# In Qiskit, we use a _quantum register_ to store our qubits.

# In[ ]:


from qiskit import QuantumRegister, ClassicalRegister, QuantumCircuit


def create_circuit():
    # Quantum register with 2 qubits
    qreg = QuantumRegister(2)
    circuit = QuantumCircuit(qreg) # ql-unmeasurable-qubits

    # Apply H gate to all qubits
    circuit.h(qreg)
    return circuit
