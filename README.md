# Deep Learning for Symbolic Mathematics in Rust

A reimplementation of Lample & Charton (2019) Deep Learning for Symbolic Mathematics, using Rust instead of Python. This attempts to be an idiomatic port to Rust but preserves much of the architecture of the original repository.

## Usage

Make sure you have the Rust build tools first. Get them [here](https://www.rust-lang.org/tools/install).

### Compiling

``` sh
cargo build --release
```

### Generate dataset

``` sh
./target/release/deepmath --generate
```

### Make model

``` sh
./target/release/deepmath --make
```

### Run model and predict

``` sh
./target/release/deepmath --predict
```
