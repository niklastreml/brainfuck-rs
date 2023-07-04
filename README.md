# brainfuck-rs
This crate provides a proc-macro for transpiling brainfuck code into rust, at compile-time. It allows writing fast brainfuck, by relying on optimizations in LLVM and rustc.

## Usage

```rust
use brainfuck::brainfuck;

fn main() {
    brainfuck!(
        "
        >++++++++[<+++++++++>-]<.
        >++++[<+++++++>-]<+.
        +++++++..
        +++.
        >>++++++[<+++++++>-]<++.
        ------------.
        >++++++[<+++++++++>-]<+.
        <.
        +++.
        ------.
        --------.
        >>>++++[<++++++++>-]<+.
        "
    );
}
```