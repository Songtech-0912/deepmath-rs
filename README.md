# Deep Learning for Symbolic Mathematics in Rust

A reimplementation of Lample & Charton (2019) Deep Learning for Symbolic Mathematics, using Rust instead of Python. This attempts to be an idiomatic port to Rust; while the core functionality is preserved, the implementation is very different.

## Usage

Make sure you have the Rust build tools first. Get them [here](https://www.rust-lang.org/tools/install).

### Compiling

``` sh
cargo build --release
```

### Get dataset

The original repository provided tools to generate data on the fly. However, because data generation can take quite a long time, I've decided to skip this and use the original paper's publicly-available dataset instead.

``` sh
./target/release/deepmath --prepare
```

Note that this does _not_ generate the data set from scratch, it instead downloads the pre-built dataset tarball from Facebook AI to a `deepmath_data` folder in your current directory.

### Train model

``` sh
./target/release/deepmath --train
```

This saves the trained model to a `.dat` file in `./deepmath_model/model.dat`, which will also be in your current directory. Note that the training will be CPU-only. Training parameters cannot be modified.

### Run model and predict

``` sh
./target/release/deepmath --predict
```

This loads the built model, and if loading is successful, it starts the WebView UI for using the model with a Jupyter-style interface. The UI will allow you to inspect the model, as well as using the model to solve integration problems and differential equations.

There is a default set of problems that Deepmath will try to solve; in addition to those, you can choose to input custom ODEs and functions to solve, using the Web UI.
