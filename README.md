# bfcc

A Brainfuck interpreter, transpiler and inspector written in Rust

### Usage

#### Installation

You can install the newest published version of the binary by running:

```bash
cargo install bfcc
```

#### Commands

- `bfcc transpile` - a Brainfuck to C optimizing transpiler
- `bfcc run` - a Brainfuck interpreter
- `bfcc spy` - a runtime inspector and soon to be debugger

### Optimization

During transpilation the AST gets converted into an intermediary representation that gets optimized in multiple passes.

Although I haven't tested these optimizations enough to provide any sort of guarantee, these optimizations all seem to generate functionally equivalent programs.

#### Implementation status

|     | Strategy                            | File                   |
| --- | ----------------------------------- | ---------------------- |
| ✅  | Fusing increment/decrement commands | `fuse_add.rs`          |
| ✅  | Fusing movements                    | `fuse_movements.rs`    |
| ✅  | Deferring movements                 | `defer_movements.rs`   |
| ✅  | Unrolling zero/clear loops          | `unroll_zero_loops.rs` |
|     | Unrolling copy loops                |                        |
|     | Unrolling multiplication loops      |                        |

<sup>All transforms can be found under `src/transpiler/transforms`</sup>

#### References

These websites have served as great references during development:

- [**Calmer than you are** - Brainfuck optimization strategies](http://calmerthanyouare.org/2015/01/07/optimizing-brainfuck.html)
- [**Project Nayuki** - Optimizing brainfuck compiler](https://www.nayuki.io/page/optimizing-brainfuck-compiler)

### Todo

- [x] Start implementing optimizations
- [ ] Implement breakpoints, single stepping and register peeking in debugger
- [ ] Add a command that generates LLVM IR, and compiles to a binary

<h3 align="center">
🧠 🦀
</h3>
