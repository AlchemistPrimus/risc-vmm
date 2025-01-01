# RISC Virtual Microprocessor Machine (RISC-VMM)

This is a RISC-V based CPU emulator software.It assumes a relaxed version of RV32E CPU's instruction set architecture for resource 
constrained embedded hardware, emulating how machine code is executed to provide an abstraction of how software controls the CPU 
hardware of a computer.

## Short glossary:   

#### Instructions   
These are primitive CPU commands. They loosely follow RISC-V instruction set architecture (RV32E).
They include moving data, working with memory and primitive arithmetic operations, e.g add, sub, mul, div, lw, sw   

#### Machine code   
Code that our CPU directly processes. Each instruction is encoded in 32 bits (4 bytes).   

#### Opcode   
This is a number that maps to an operation. It is important to help determine how the instruction will be executed

#### Registers   
Fixed set of temporary storage placeholders for operands in the CPU. Here, we have 16 general purpose registers 
which are x0, x1, ..., x15 and other special registers like the program counter(pc) which stores an instruction pointer 
which points to the location in RAM where it's going to fetch the next instruction.   

Uses load/store memory architecture. Instructions can only operate directly on the data in the registers. 
Dedicated instructions, lw and sw are used to load or store data from or to the processor's internal registers.  
The processor can only execute instructions on the data stored in the registers as operands, in a fetch-decode-execute cycle.

## Dependencies.   
- Rust  
- rustc  
- Cargo  
