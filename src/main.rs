use anyhow::Result as AnyhowResult;
use clap::{App, Arg};
use colored::*;
use deepmath::data_retrieve::checks;
use deepmath::data_retrieve::download;
use deepmath::data_retrieve::unarchive;
use std::env;
use std::path::Path;
use termimad::*;

// Just in case, to stay on the safe side
const _THIS_ALGORITHM_BECOME_SKYNET: bool = false;

// To make sure that nothing weird happens from 4am coding
const _IGNORE_4AM_CODING_BUGS: bool = true;

// Deepmath markdown tutorial
const TUTORIAL: &str = r#"
**Deepmath tutorial**

Hello from Deepmath! Deepmath is an alternative implementation of
*"Deep Learning for Symbolic Mathematics"* in Rust. Its neural network
can solve a variety of integration and differentiation problems.

Follow the easy steps below to get started.

**Step 1: Get the dataset**
```
deepmath --prepare --debug
```

**Step 2: Train the model**
```
deepmath --train --debug
```

By default Deepmath will train its model to `$TEMP_DIR/deepmath_data/model.dat`.
This should be `/tmp` on Mac and Linux, and `%userprofile%\AppData\Local\Temp`
for Windows. You can use the `--train_to <yourmodel.dat>` option instead to train
to the location of your choice.

**Step 3: Use the model to solve**

You can let deepmath automatically find its trained model, typically saved
to `$TEMP_DIR/deepmath_data/model.dat`:
```
deepmath --predict --input "equations.yml" --debug
```

Or you can manually specify a model path:
```
deepmath --load "model.dat" --input "equations.yml" --predict --debug
```

Note that you can choose to not specify the `--input` option. If that
is the case, then Deepmath will solve a default set of equations from
its built-in catalogue.

Once the equations are solved, deepmath will open a WebView window
showing the final results rendered in a Jupyter-style interface. If
you don't want the WebView UI, you can instead output to static HTML
files:

```
deepmath --input "equations.yml" --predict --to_file output.html --debug
```
"#;

fn show_tutorial(skin: &MadSkin) {
    println!("{}", skin.inline(TUTORIAL));
}

fn run(is_debug: bool) -> AnyhowResult<()> {
    log::info!("Beginning data download");
    let datafolder = download::init_download_dir(is_debug)?;
    let data = download::get_data(is_debug)?;
    let _ = download::get_filesize(is_debug)?;
    let data_filename = download::get_filename(is_debug)?;
    let data_full_path = datafolder.join(data_filename);
    let _ = download::contents_to_file(data, &data_full_path, is_debug);
    log::info!("Finished data download");
    log::info!("Beginning data decompress");
    let _ = unarchive::decompress(&data_full_path, &datafolder, is_debug);
    log::info!("Finished data decompress");
    Ok(())
}

// Creates and trains model
// this has an optional
fn train(write_file: Option<&Path>, is_debug: bool) -> AnyhowResult<()> {
    // Check that the dataset is already present in /tmp/deepmath
    log::info!("Finding downloaded dataset");
    let default_output = &std::env::temp_dir().join("deepmath_data/model.dat");
    let output_file = write_file.unwrap_or(&default_output);
    let mode = match write_file {
        Some(_) => "debug",
        None => "standard",
    };
    log::info!("Training under {} mode to {}", mode, &output_file.display());
    if checks::data_present()? {
        // Do the machine learning stuff
        // callsomefunction(is_debug)
    } else {
        log::error!(
            "The data wasn't located in {:?}, try running {} again",
            checks::data_location()?,
            "deepmath --prepare".green()
        );
    }
    Ok(())
}

fn main() -> AnyhowResult<()> {
    // Handle empty argument
    let mut is_run = false;
    let args: Vec<String> = env::args().collect();
    // Custom welcome messsage if user doesn't have any args
    if args.len() == 1 {
        println!(
            "{}",
            format!(
                "\n{} Run {} to start.\n",
                "Welcome to Deepmath!".bold(),
                "deepmath --tutorial".green()
            )
        );
    } else {
        is_run = true
    }
    if is_run {
        // Arguments here are slightly trimmed from the
        // original
        // for the sake of brevity: notably, I have removed
        // all of the model arguments and AMP-related arguments
        let matches = App::new("Deepmath")
            .version("1.0")
            .about("Deep learning model for symbolic mathematics in Rust.")
            .arg(
                Arg::new("debug")
                    .short('d')
                    .long("debug")
                    .help(
                        "Toggles debug mode for verbose output. \
                    This MUST be run with another command line argument.",
                    ),
            )
            .arg(
                Arg::new("prepare")
                    .short('p')
                    .long("prepare")
                    .help("Download training and test data from Facebook AI"),
            )
            .arg(
                Arg::new("tutorial")
                    .short('q')
                    .long("tutorial")
                    .help("Shows the Deepmath tutorial"),
            )
            .arg(
                Arg::new("train")
                    .short('t')
                    .long("train")
                    .help("Train model and save model to the default location")
            )
            .arg(
                Arg::new("train_to")
                    .short('e')
                    .long("train_to")
                    .help("Train model and save model to a specified location")
                    .takes_value(true),
            )
            .arg(
                Arg::new("predict")
                    .short('c')
                    .long("predict")
                    .help(
                        "Uses trained model to predict. Must be \
                    used with --load for the trained model to load, \
                    and optionally with --input for an input \
                    equation list to solve",
                    )
                    .takes_value(true),
            )
            .arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help(
                        "Reads equation from a .yml file to solve. \
                    Must be used with --predict and --load.",
                    )
                    .takes_value(true),
            )
            .arg(
                Arg::new("load")
                    .short('u')
                    .long("load")
                    .help(
                        "Loads a pre-trained model  \
                    from a .dat model archive. \
                    Muse be used with --predict and can \
                    be optionally used with --input",
                    )
                    .takes_value(true),
            )
            .arg(
                Arg::new("to_file")
                    .short('o')
                    .long("to_file")
                    .help(
                        "Outputs rendered prediction results \
                        to an html file of your choice",
                    )
                    .takes_value(true),
            )
            .get_matches();
        // Let's just...ignore the fact that we should check the options
        // before we start
        // check_model_params(matches)
        simple_logger::init().unwrap();
        let is_show_tutorial = matches.is_present("tutorial");
        let is_prepare = matches.is_present("prepare");
        let is_debug_mode = matches.is_present("debug");
        let is_train = matches.is_present("train");
        let is_train_to_file = matches.is_present("train_to");

        if is_show_tutorial {
            let skin = MadSkin::default();
            show_tutorial(&skin);
        }
        if is_prepare {
            if is_debug_mode {
                run(true)?;
            } else {
                run(false)?;
            }
        }
        // regular train, to default path
        if is_train {
            if is_debug_mode {
                train(None, true)?;
            } else {
                log::info!("Sorry, training to file (standard) is not yet done");
            }
        }
        // train to specific path
        if is_train_to_file {
            if is_debug_mode {
                let model_file = Path::new(matches.value_of("train_to").unwrap());
                train(Some(model_file), true)?;
            } else {
                train(None, true)?;
            }
        }
    }
    Ok(())
}
