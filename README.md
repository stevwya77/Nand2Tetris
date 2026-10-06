# Nand2Tetris

A computer built from a single NAND gate up, following *The Elements of Computing Systems* by Noam Nisan and Shimon Schocken (MIT Press). Projects 1 through 5 are written in the course's hardware description language (HDL). Project 6 is an assembler written in Rust.

## What's here

| Project | What I built |
|---|---|
| [1. Boolean logic](project1) | And, Or, Not, Xor, multiplexers and demultiplexers, plus 16-bit and multi-way versions, all from NAND |
| [2. Boolean arithmetic](project2) | Half adder, full adder, 16-bit adder, incrementer, and the ALU |
| [3. Memory](project3) | Bit, register, program counter, and RAM from 8 words up to 16K |
| [4. Machine language](project4) | `Mult.asm` (multiplication by repeated addition) and `Fill.asm` (keyboard-driven screen fill) in Hack assembly |
| [5. Computer architecture](project5) | The Hack CPU, memory map, and the full computer |
| [6. Assembler](project6) | A two-pass Hack assembler in Rust |

## The assembler

The assembler translates Hack assembly (`.asm`) into 16-bit machine code (`.hack`).

- **Pass one** walks the file and records every label, such as `(LOOP)`, with the address of the instruction that follows it.
- **Pass two** translates each instruction. A-instructions (`@value`) become a 0 followed by a 15-bit address, and new variables are allocated from RAM address 16 up. C-instructions (`dest=comp;jump`) become `111` followed by the comp, dest, and jump bit fields.

The code is split into a parser (`parser.rs`), binary code tables (`code.rs`), a symbol table seeded with the predefined symbols (`symboltable.rs`), and a driver (`hackassembler.rs`).

### Build and run

```bash
cd project6
rustc -O hackassembler.rs -o hackassembler
./hackassembler path/to/Program.asm   # writes path/to/Program.hack
```

The HDL chips run in the official Nand2Tetris hardware simulator.
