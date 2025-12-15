// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

// R0 is base
@16384
D=A
@R0
M=D

// R1 is n pixels
@8192
D=A
@R1
M=D

// R2 = 0, count
@R2
M=0

(LOOP)
// if (R2==R1) reset count again for loop
@R2
D=M
@R1
D=D-M
@CHECK
D;JEQ

@KBD
D=M
@WHITE
D;JEQ
// RAM[R0+i] = -1
@R0
D=M
@R2
A=D+M
M=-1

@KBD
D=M
@34
D;JGT
(WHITE)

// RAM[R0+i] = 0
@R0
D=M
@R2
A=D+M
M=0

// i = i + 1
@R2
M=M+1

// goto loop
@LOOP
0;JMP


(CHECK)
@R2
M=0
@LOOP
0;JMP

(END)
@END
0;JMP