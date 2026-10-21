# Ihit Compiler

A modern compiler for the `.ihit` programming language with a custom intermediate representation (IR), optimization passes, and x86-64 code generation.

## Overview

This compiler implements a complete compilation pipeline from source code to machine code:

1. **Lexical Analysis** - Tokenizes the source code
2. **Parsing** - Builds an Abstract Syntax Tree (AST)
3. **Semantic Analysis** - Type checking and semantic validation
4. **IR Generation** - Converts AST to custom Intermediate Representation
5. **Optimization** - Applies various optimization passes based on flags
6. **Code Generation** - Emits x86-64 assembly
7. **Assembly & Linking** - Creates executable binaries

## Why Custom IR?

The compiler uses a custom IR implementation instead of LLVM/Inkwell for several reasons:

- **Version Independence**: No dependency on specific LLVM versions
- **Learning & Control**: Full understanding and control over the IR design
- **Simplicity**: Tailored to the specific needs of the .ihit language
- **Educational Value**: Demonstrates compiler design principles

## Features

- **Custom IR Design**: SSA-like intermediate representation with basic blocks
- **Optimization Passes**: Constant folding, dead code elimination, constant propagation, instruction combining, loop simplification
- **x86-64 Code Generation**: Direct assembly output for Linux x86-64
- **Optimization Levels**: 4 levels (0-3) with increasing optimization
- **Debug Mode**: Token stream, AST, and IR inspection
- **Pretty Printing**: Formatted output for debugging

## Installation

### Prerequisites

- Rust 1.70 or later
- GNU assembler (as)
- GNU linker (ld)

### Build

```bash
cargo build --release
```

## Usage

### Basic Compilation

```bash
./target/release/compiler <file.ihit>
```

### With Optimization

```bash
./target/release/compiler -O2 <file.ihit>
```

### Debug Mode

```bash
./target/release/compiler -d <file.ihit>
```

### Pretty Debug Mode

```bash
./target/release/compiler -df <file.ihit>
```

### Command Line Options

- `-d`: Enable debug mode (shows tokens, AST, IR)
- `-f`: Enable pretty printing (requires debug mode)
- `-df` or `-fd`: Enable both debug and pretty mode
- `-O[0/1/2/3]`: Set optimization level (higher = more optimized)
- `-E=(ENV_VAR=VAL,...)`: Set environment variables
- `<FILES>`: Input files (.ihit or .rs extension)

### Optimization Levels

- **O0**: No optimizations (default)
- **O1**: Basic optimizations (constant folding, dead code elimination)
- **O2**: Default optimizations (adds constant propagation, instruction combining)
- **O3**: Aggressive optimizations (adds loop simplification, memory-to-register promotion)

## Language Syntax

### Variables

```ihit
let x = 42;
let mut y = 10;
let z: int = 100;
```

### Functions

```ihit
fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() {
    let result = add(5, 3);
}
```

### Control Flow

```ihit
if (x > 0) {
    print("positive");
} else {
    print("non-positive");
}

while (x > 0) {
    x = x - 1;
}

for i in 0..10 {
    print(i);
}

loop {
    break;
}
```

### Operators

- Arithmetic: `+`, `-`, `*`, `/`, `%`, `^` (pow)
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`
- Bitwise: `&`, `|`, `~`, `<<`, `>>`

## API Documentation

### Main Compiler API

#### `Compiler`

The main compiler structure that orchestrates the compilation pipeline.

```rust
use crate::Compile::compiler::Compiler;

let mut compiler = Compiler::new();
compiler.Run()?;
```

#### Methods

- `new()` - Creates a new compiler instance
- `Run(&mut self) -> CompilerReturn<bool>` - Executes the compilation pipeline

### Custom IR API

#### `Module`

Represents a compilation unit containing functions and global data.

```rust
use crate::CustomIR::ir::Module;

let mut module = Module::new("my_module".to_string());
module.add_function(function);
```

#### Methods

- `new(name: String) -> Self` - Creates a new module
- `add_function(&mut self, func: Function)` - Adds a function to the module
- `get_function(&self, name: &str) -> Option<&Function>` - Retrieves a function
- `declare_external_function(&mut self, name: String, return_type: IrType, params: Vec<IrType>)` - Declares an external function

#### `Function`

Represents a function with basic blocks.

```rust
use crate::CustomIR::ir::{Function, IrType};

let mut func = Function::new("main".to_string(), IrType::Void);
func.add_param("x".to_string(), IrType::I64);
```

#### Methods

- `new(name: String, return_type: IrType) -> Self` - Creates a new function
- `add_param(&mut self, name: String, ty: IrType)` - Adds a parameter
- `get_block(&mut self, name: &str) -> &mut BasicBlock` - Gets or creates a basic block
- `add_block(&mut self, name: String)` - Adds a new basic block

#### `BasicBlock`

A sequence of instructions with single entry/exit.

```rust
use crate::CustomIR::ir::{BasicBlock, Instruction};

let mut block = BasicBlock::new("entry".to_string());
block.add_instruction(Instruction::Const { ... });
```

#### Methods

- `new(name: String) -> Self` - Creates a new basic block
- `add_instruction(&mut self, instr: Instruction)` - Adds an instruction
- `is_terminated(&self) -> bool` - Checks if block ends with terminator

#### `Instruction`

IR instruction types.

Variants:
- `Alloca { dest, ty }` - Stack allocation
- `Store { src, dest }` - Store to memory
- `Load { dest, src, ty }` - Load from memory
- `Binary { dest, op, left, right, ty }` - Binary operation
- `Unary { dest, op, operand, ty }` - Unary operation
- `Const { dest, value }` - Constant value
- `Call { dest, func, args }` - Function call
- `Return { value }` - Return from function
- `BranchCond { cond, true_block, false_block }` - Conditional branch
- `Branch { block }` - Unconditional branch
- `Phi { dest, incoming, ty }` - Phi node (SSA)
- `Cast { dest, src, from, to }` - Type cast
- `GetElementPtr { dest, base, index }` - Pointer arithmetic
- `Cmp { dest, op, left, right }` - Comparison

### IR Generator API

#### `IRGenerator`

Converts AST to custom IR.

```rust
use crate::IRGen::irgen::IRGenerator;

let mut ir_gen = IRGenerator::new("input.ihit");
ir_gen.set_optimization_level(2);
ir_gen.generate(&ast)?;
```

#### Methods

- `new(fname: &str) -> Self` - Creates a new IR generator
- `set_optimization_level(&mut self, level: usize)` - Sets optimization level (0-3)
- `generate(&mut self, code: &Code) -> CodegenReturn<()>` - Generates IR from AST
- `print_ir(&self)` - Prints IR to stdout
- `get_ir_string(&self) -> String` - Returns IR as string
- `get_module(&self) -> &Module` - Gets the IR module
- `get_module_mut(&mut self) -> &mut Module` - Gets mutable IR module

### Optimizer API

#### `Optimizer`

Applies optimization passes to IR.

```rust
use crate::Optimizer::optimizer::Optimizer;

let optimizer = Optimizer::new(2);
optimizer.optimize(&mut module);
```

#### Methods

- `new(level: usize) -> Self` - Creates optimizer with given level
- `optimize(&self, module: &mut Module)` - Runs optimization passes

#### Optimization Passes

- **Constant Folding**: Evaluates constant expressions at compile time
- **Dead Code Elimination**: Removes unused instructions
- **Constant Propagation**: Replaces variables with constant values
- **Instruction Combining**: Combines instruction sequences
- **Loop Simplification**: Simplifies loop structures
- **Memory to Register Promotion**: Promotes stack variables to registers

### Code Generator API

#### `CodeGenerator`

Converts custom IR to x86-64 assembly.

```rust
use crate::Codegen::codegen::CodeGenerator;

let mut codegen = CodeGenerator::new(module, 2);
let assembly = codegen.generate()?;
codegen.write_to_file("output.s")?;
```

#### Methods

- `new(module: Module, optimization_level: usize) -> Self` - Creates code generator
- `generate(&mut self) -> CodegenReturn<String>` - Generates assembly code
- `write_to_file(&self, filename: &str) -> CodegenReturn<()>` - Writes assembly to file

## Examples

### Example 1: Simple Arithmetic

```ihit
fn main() {
    let a = 10;
    let b = 20;
    let c = a + b;
    let d = a * b;
}
```

Compile:
```bash
./target/release/compiler example1.ihit -O2
```

### Example 2: Function with Parameters

```ihit
fn factorial(n: int) -> int {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

fn main() {
    let result = factorial(5);
}
```

### Example 3: Loops

```ihit
fn main() {
    let sum = 0;
    for i in 1..10 {
        sum = sum + i;
    }
}
```

### Example 4: String Handling

```ihit
fn main() {
    let message = "Hello, World!";
}
```

## Architecture

### Compilation Pipeline

```
Source Code (.ihit)
    ↓
Lexer (Tokeniser)
    ↓
Parser (AST)
    ↓
Semantic Analyzer
    ↓
IR Generator (CustomIR)
    ↓
Optimizer (Optimizer)
    ↓
Code Generator (x86-64)
    ↓
Assembler (as)
    ↓
Linker (ld)
    ↓
Executable
```

### Module Structure

- `src/Ast.rs` - Abstract Syntax Tree definitions
- `src/Lexer_Tok.rs` - Token definitions
- `src/Tokeniser.rs` - Lexical analyzer
- `src/Parser.rs` - Parser
- `src/Semantic_Analysis.rs` - Semantic analyzer
- `src/CustomIR.rs` - Custom IR representation
- `src/IRGen.rs` - IR generator
- `src/Optimizer.rs` - Optimization passes
- `src/Codegen.rs` - Code generator
- `src/Compile.rs` - Main compiler API
- `src/Frontend.rs` - Frontend orchestration
- `src/Helper.rs` - Helper utilities
- `src/Errors.rs` - Error types
- `src/main.rs` - Entry point

## Error Handling

The compiler uses comprehensive error types:

- `ParserError` - Syntax errors during parsing
- `Semantic_err` - Type checking and semantic errors
- `CodegenError` - IR generation and code generation errors
- `CompilerError` - High-level compiler errors

## Contributing

Contributions are welcome! Please ensure:

- Code follows Rust style guidelines
- All tests pass
- New features include documentation
- Changes don't break existing functionality

## License

GPL-3.0-only - See LICENSE file for details

## Author

Ihit Acharya - Copyright (C) 2025

## Future Work

- [ ] Add support for structs and enums in IR
- [ ] Implement more optimization passes
- [ ] Add register allocation
- [ ] Support for more architectures (ARM, etc.)
- [ ] Standard library
- [ ] Better error messages
- [ ] Incremental compilation
- [ ] LTO (Link Time Optimization)
