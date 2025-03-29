# LC-3 Virtual Machine

This project aims to implements a virtual machine for the LC-3 (Little Computer 3) architecture. Built with Rust for memory safety and performance.

## Overview

The LC-3 VM is written in Rust and simulates the basic functionality of the LC-3 architecture, including:
- 10 16-bit registers (R0-R7 general purpose, R8 as PC, R9 as flags)
- 64KB of memory (16-bit address space)
- Basic instruction set execution
- Condition flags (Positive, Zero, Negative)


## Features

- **Registers**: 10 16-bit registers
  - R0-R7: General purpose registers
  - R8: Program Counter (PC)
  - R9: Flag register
- **Memory**: 64KB (2^16) addressable memory locations
- **Condition Flags**: Positive (POS), Zero (ZRO), Negative (NEG)
- **Instruction Set**: Supports basic LC-3 opcodes including:
  - BR, ADD, LD, ST, JSR, AND, LDR, STR
  - RTI, NOT, LDI, STI, JMP, RES, LEA, TRAP

## Getting Started

### Prerequisites
- Rust programming language (latest stable version)
- Cargo (Rust's package manager)

### Installation
1. Clone the repository:
```bash
git clone [repository-url]
cd lc3-vm
```

Build and run:

```bash
cargo run
```

### Usage

The VM currently initializes with a Program Counter set to 0x3000, the Zero flag set. An infinite execution loop fetching instructions from memory.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for:

- Bug fixes
- Feature implementations
- Documentation improvements
- Performance optimizations

## Acknowledgments
Inspired by the LC-3 architecture developed at the University of Texas at Austin.
