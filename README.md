Collection of quantum data-structure and algorithims, just a simple pet project based on the content of "Quantum Computing for Computer Scientistis" by Yonofsky.

#### Includes:
- Complex Numbers
- Static sized and dynamic sized complex vectors and matrices.
- Quantum states, quantum gates.
- Basic quantum assembly language emulation.
- Various algorithms
    - Deutsch-Josza
    - Grover Search
    - Simon's Periodicity
    - Shor's
    - BB84 & BB96 Key Exchange

#### Possible Improvements
- I did not really design this with speed in mind,as the main bottleneck for quantum simulation is limited memory. So lots of places where speed can be improved.

- The main bottleneck is the huge quantum oracles, which wouldnt exist as their own gates in real quantum circuits anyway, so implementing a way of applying gates to portions of states without tensoring with a huge identity matrix would allow for much larger inputs. For example for Shor's algorithim, inputs larger than 16, require 10 + 5 qubits in the main circuit, so the main oracle would have 2GB of data, even though its mostly 0's.

