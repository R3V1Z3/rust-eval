# rust-eval
Dead simple command-line expression evaluator in Rust as an example project

## Compiling

- Ensure [Rust](https://www.rust-lang.org/en-US/) is installed
- Clone the repo: `git clone https://github.com/Ugotsta/rust-eval.git`
- Open the /rust-eval/ folder
- Built with Cargo: `cargo build --release`

## Usage

Once compiled, find the binary file in the /target/release/ folder and run it with math expressions as parameters.

`./eval 5+5`
`./eval 5+(5*5)-5`

## Dependencies

eval: https://crates.io/crates/eval
