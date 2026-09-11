# The Unreasonable Computer Project

I'm building a computer from scratch. Because apparently knowing how to use computers wasn't enough. Not literally mining silicon and manufacturing transistors in my backyard. I'm not that unemployed. The idea is to start from the smallest useful thing I can understand a bit, and slowly build my way up until there's a machine that can actually run programs. I wanted to know what happens underneath all the nice abstractions. So instead of doing something reasonable with my time, I'm starting with a bit and working my way up to an operating system. **I want to know what's underneath it.**

```
BIT
  -> LOGIC GATES
  -> ADDER
  -> ALU
  -> REGISTERS
  -> RAM
  -> CPU
  -> INSTRUCTION SET
  -> ASSEMBLER
  -> PROGRAM
  -> COMPILER
  -> OS
```

> At some point I will probably ask myself why I didn't just install Linux and move on with my life. Too late.

A computer is basically a gigantic tower of abstractions. At the top: "hello world". At the bottom: electricity. And somewhere in between, humanity decided that the best way to organize those electrons was to invent:

compilers, operating systems, instruction sets, virtual memory, cache hierarchies and then spend the next 70 years arguing about programming languages. **I want to see how that whole mess is actually built.**

---

## The Goal

The goal isn't to build the fastest computer. It isn't to compete with Intel. It definitely isn't to run Cyberpunk. The goal is to reach the point where I can look at: 1011010010110100 and not immediately think: "Cool. Anyway." **I want to understand what those bits mean, where they go, what hardware moves them, what instruction they represent, and how that instruction eventually contributes to an actual program.**

> **Basically: I want the computer to stop being magic.**

And yes, it's going to break A lot. The CPU will probably not work. Then it will almost work. Then it will work except for one instruction. Then that instruction will break something else. Then the compiler will generate something the CPU doesn't understand. Then the kernel will crash. Then I'll discover that the original problem was a single bit flipped somewhere 400 lines below the thing I was debugging. And honestly? That's kind of the point. **Because every bug is another excuse to understand the machine a little better.**

> **I'm not trying to build the next Intel. I'm trying to understand what the hell Intel had to build.**

**Starting with: 0, End with: Hello, world! Everything in between is the fun part.**

---

## Will It Be Useful?

Will this be useful? Probably. Will it be practical? Absolutely not. Could I spend this time learning something more marketable? Definitely. Could I buy a Raspberry Pi and accomplish 90% of what I want? Yes. Will that stop me? No. I have already made a directory called 01-bits. **There is no going back.**

---

## Final Warning

> This repository contains an unnecessary amount of ambition, questionable engineering decisions, Rust, binary numbers, hexadecimal numbers, assembly, memory bugs, CPU bugs, compiler bugs, and probably compiler bugs caused by CPU bugs. There may also be documentation written at 3 AM, an unreasonable amount of println!(), several decisions that seemed like a good idea at the time, and, if the stars align and I haven't accidentally invented a new form of computer failure, potentially one functioning computer.

**Proceed at your own risk. If something starts working, do not touch it.**