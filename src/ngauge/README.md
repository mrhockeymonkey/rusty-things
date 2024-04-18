# NGauge


https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md#from-rust
https://github.com/bytecodealliance/wasmtime/blob/main/examples/wasi/main.rs
https://docs.wasmtime.dev/examples-rust-wasi.html
Initial idea to use WASI

```bash
cargo +nightly run -p ngauge-wasi
```

question: can io all be done with files instead of stdin/stdout


component model approach

```bash
rustc --version
# rustc 1.77.2 (25ef9e3d8 2024-04-09)

```

## Rust components

See: https://component-model.bytecodealliance.org/language-support/rust.html

```bash
#  create a new rust wasi component
cargo install cargo-component
cargo component new rs-gauge --lib & cd rs-gauge

# now you can update the world.wit and implement any functions, I added:
# export measure: func(assets: list<string>) -> u32;

# to build
cargo component build --release
   #Compiling rs-gauge v0.1.0 (/home/scott/code/rusty-things/src/ngauge/examples/rs-gauge)
   # Finished release [optimized] target(s) in 0.12s
   # Creating component /home/scott/code/rusty-things/src/ngauge/target/wasm32-wasi/release/rs_gauge.wasm

```

# Python Components

See: https://component-model.bytecodealliance.org/language-support/python.html

```bash
pip install componentize-py
mkdir py-gauge & cd py-gauge

# create binding
componentize-py --wit-path ./wit/world.wit --world example bindings .

# now you can implement the generate pythong class, see py-gauge.py

# compile
componentize-py --wit-path ../../add/wit/world.wit --world example componentize py-gauge -o py-gauge.wasm
# Component built successfully

```