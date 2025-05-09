# rust-ASM-compiler

This rust program converted custom assembly instructions to usable machine code
for a custom CPU in Logisim Evolution.

![Overview of a CPU in Logisim](./main_final.png)

Each component here was developed in a specific lab period over the course of a
semester. Below is a list of components and their function.

![A register bank with 32 registers](./lab5pics/Registers.png)

This register bank held 32 16-bit values and was capable of reading 8 of them in
one cycle, but for writes the limit was 4 per cycle.

![A set of counters](./lab5pics/Counters.png)

2 Counters, one set to count up, the other down. Each will reset after
overflowing.

![A set of math logical blocks](./lab5pics/ARITHMETIC_BLOCK.png)

These pre-built logical blocks were how we handled math.

![A set of splitters performing bitwise OR functions on data](./lab5pics/LOGICAL_BLOCK.png)

These sets of gates performed bitwise logic on 2 inputs.

These 2 components were eventually simplified into the CORE shown below.

![A data pipeline with inputs for each possible operation](./lab5pics/CORE.png)

![A set of checks going into shared memory](./lab5pics/Store_Check.png)

These checks were performed on each core to prevent a race condition from
occurring.

![A Vector Register bank, holding 8 registers](./lab5pics/Vector_Unit.png)

This vector unit holds 8 64-bit values, assembled from shared memory and
represented as 4 16-bit values combined.

![A circuit that performs a dot product operation](./lab5pics/Vector_ALU.png)

This block performs the arithmetic necessary to run a dot product operation on 8
16-bit inputs, with the output being 1 64-bit value.
