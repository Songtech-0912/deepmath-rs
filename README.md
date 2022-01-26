# Deep Learning for Symbolic Mathematics in Rust

A reimplementation of Lample & Charton (2019) Deep Learning for Symbolic Mathematics, using Rust instead of Python. This attempts to be an idiomatic port to Rust but preserves much of the architecture of the original repository.

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

Note that this does _not_ generate the data set from scratch, it instead downloads the pre-built dataset tarball from the Facebook AI servers to your `/tmp` folder. If the dataset is already downloaded, it will not run.

### Train model

``` sh
./target/release/deepmath --train
```

This saves the trained model to a `.dat` file in `$TEMP_DIR/deepmath_data.dat` (this resolves to `%userprofile%\AppData\Local\Temp` on Windows, `/tmp` on Mac and same for Linux), and reminds you to run `deepmath --prepare` if the dataset wasn't already downloaded or wasn't downloaded to the right place. Note that the training will be CPU-only.

### Run model and predict

``` sh
./target/release/deepmath --input "equations.yml" --predict
```

This loads the built model (reminding you to do the data loading and training steps if the model doesn't exist). It takes in a `yml` file consisting of integration problems and first- and second-order differential equations to solve. There is a `equations.yml` pre-provided in the repository; if not, Deepmath will try to solve a default selection of integral and differential equations.

By default Deepmath will display the solving results using (Ka)TeX in a Webview window, Jupyter-style, using its web server. You should see something like this:

```
Training completed! Wrote results to local directory: /home/user/symbolic-rust/output/html
Starting server at http://localhost:8888 and creating webview window...
Pointing webview to server...

Window is open. Use Ctrl-C to shut down this window.
```

If you want to output to HTML files directly, without the webview window, run:

``` sh
./target/release deepmath --load_from "model.dat" --predict --to_file output.html
```

