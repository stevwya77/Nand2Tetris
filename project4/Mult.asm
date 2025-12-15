// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
// The algorithm is based on repetitive addition.

// D: data register
// A: address register
// M: selected RAM register

// default R2 = 0
@R2
M=0
// start counter at 1
@i
M=1

(LOOP)
// check for (i > R1), if yes goto stop
@i
D=M
@R1
D=D-M
@STOP
D;JGT

// else, increment count by 1
@i
M=M+1

// add R0 to R2
@R0
D=M
@R2
M=D+M

// goto loop to continue until counter reaches R1
@LOOP
0;JMP

(STOP)
(END)
@END
0;JMP