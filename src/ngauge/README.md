# NGauge

This is an example of using wasi component in a plugin context. 

In this example app multiple wasi components in different languages implement the `measure: func(assets: list<string>) -> u32` interface. 

Each of these components is compiled and hosted by `MetricHost` in a loop. 


```bash
cargo run -p ngauge-wasi

#MetricHost - Adding wasi component: ../target/wasm32-wasi/release/rs_gauge.wasm
#MetricHost - Adding wasi component: ../examples/py-gauge/py-gauge.wasm
#Calling collection - 1
#rs-gauge will count how many strings were given
#got foo
#got bar
#got baz
#MetricHost - Component 1 Measure returned 3
#py-gauge will measure how many strings begin with 'b'
#foo
#bar
#baz
#MetricHost - Component 2 Measure returned 2
#Sleeping 5s

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

## Python Components

See: https://component-model.bytecodealliance.org/language-support/python.html

```bash
pip install componentize-py
mkdir py-gauge & cd py-gauge

# create binding
componentize-py --wit-path ./wit/world.wit --world example bindings .

# now you can implement the generate pythong class, see py-gauge.py

# compile
componentize-py --wit-path ./wit/world.wit --world example componentize py-gauge -o py-gauge.wasm
# Component built successfully

```

## Useful Links

https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md#from-rust
https://github.com/bytecodealliance/wasmtime/blob/main/examples/wasi/main.rs
https://docs.wasmtime.dev/examples-rust-wasi.html
https://component-model.bytecodealliance.org/language-support/rust.html#exporting-an-interface-with-cargo-component
