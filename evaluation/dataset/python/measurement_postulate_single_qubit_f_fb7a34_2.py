# https://github.com/a-liv-e/QM1-Exercises/blob/4258d0cf9d9ada5aae9759ef3709de11e677010b/Measurement-postulate-Single-qubit-F.ipynb
#!/usr/bin/env python
# coding: utf-8

# # Measurements on a single qubit
# 
# Quantum mechanics is often said to counterintuitive, in particular when it comes to measurements. This could well be the case if you have only trained your intuition on classical systems. This exersice is a first step towards building up your quantum intuition!
# 
# We will work on a single qubit and thereby at the same time familiarize ourseves with the basic building block of quantum computers such as IBM Q.
# 
# **Learning goal:** To gain a first expirence with the measurement postulate and start to build up your quantum intuition. Familiarize our selves with the qubit.
# 

# # The measurement postulate
# 
# Let's keep things simple and work with a two level system, given by the two states $|0\rangle$ and $|1\rangle$. More formally, we have that the two states $|0\rangle$ and $|1\rangle$ are orthonormal and span the Hilbert space. Hence any normalized state $|\psi\rangle$ in this 2d space can be written as a superposition of these two states
# 
# \begin{equation}
# |\psi\rangle= \alpha|0\rangle+\beta|1\rangle \ ,
# \end{equation}
# 
# with $|\alpha|^2+|\beta|^2=1$.
# 
# 
# In this 2d space the measurement postulate of quantum mechanics states: 
# 
# If the system is in the state $|\psi\rangle= \alpha|0\rangle+\beta|1\rangle$ then
# 
# **1.** The probability that a measuremet will yield the value $0$ respectively $1$ is given by
# 
# \begin{equation}
# P_0 = |\langle 0|\psi\rangle|^2 = |\alpha|^2  \quad {\rm and} \quad P_1 = |\langle 1|\psi\rangle|^2 = |\beta|^2
# \end{equation}
# 
# **2.** If the outcome of the measurement is 0 then the sate of the system right after the measurement is $|0\rangle$ and likevise if the outcome is $1$ the state collapses to $|1\rangle$. 
# 
# 
# But what do we measure here? Well, physical observables are given by Hermitian operators and the possible outcomes are the eigenvalues of the given Hermitian operator. Therefore we are measuring an operator, $M$, with eigenvalues $0$ and $1$ and it must be diagonal in the basis given by $|0\rangle$ and $|1\rangle$,  
# 
# \begin{equation}
# M=\left(\begin{array}{cc} 0 & 0 \\ 0 & 1 \end{array}
# \right)
# \end{equation}
# 
# Why diagonal? Well if the state of the system was $|\psi\rangle = |0\rangle$ and we where to measure $M$ then the outcome would with 100% certainty be 0 and likevise if the state of the system was $|\psi\rangle = |1\rangle$ we would get the value 1. Hence $|0\rangle$ and $|1\rangle$ are eigenstates of the Hermitian operator $M$ and $M$ is diagonal in this basis. 
# 
# **Exercise 0:** What does the states $|0\rangle$ and $|1\rangle$ look like in the basis $|0\rangle$ and $|1\rangle$?

# **Our first measurement!**
# 
# To get started let's simply put the system in the state $|0\rangle$ and then perform a measurement of the Hermitain operator $M$ for which we now know that  
# 
# \begin{eqnarray}
# M|0\rangle & = & 0|0\rangle \\
# M|1\rangle & = & 1|1\rangle 
# \end{eqnarray}

# In[ ]:


# Importing standard Qiskit libraries
from qiskit import QuantumCircuit
from math import pi


def create_circuit():
    circ = QuantumCircuit(1,3)
    circ.measure(0,0)
    circ.ry(pi/2,0)
    circ.measure(0,1) # ql-double-measurement
    circ.measure(0,2) # ql-double-measurement
    return circ
