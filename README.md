# bfcc


Brainfuck interpreter, transpiler and inspector written in Rust


### Usage

#### Installation

You can install the newest published version of the binary by running:

```bash
cargo install bfcc
```


#### Commands

- `bfcc run` - a Brainfuck interpreter
- `bfcc transpile` - a Brainfuck to C transpiler
- `bfcc spy` - a runtime inspector and soon to be debugger


### Todo

- [ ] Implement single stepping and register peeking in debugger
- [ ] Optimize code to an intermediary ast before interpreting, combining multiple commands of same type into one
- [ ] Add a command that generates LLVM IR, and compiles to a binary



<h3 align="center">
🧠 🦀
</h3>
