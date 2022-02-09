#![allow(dead_code)]
use anyhow::Result as AnyhowResult;
use clap::{App, Arg};
use colored::*;
use deepmath::data_retrieve;
use data_retrieve::checks;
use data_retrieve::download;
use data_retrieve::unarchive;
use std::env;
// use std::fs::File;
// use std::io::prelude::*;
use std::path::Path;
use termimad::*;

// Just in case, to stay on the safe side
const _THIS_ALGORITHM_BECOME_SKYNET: bool = false;

// To make sure that nothing weird happens from 4am coding
const _IGNORE_4AM_CODING_BUGS: bool = true;

// Deepmath tutorial
const TUTORIAL: &str = include_str!("./include/tutorial.md");

fn show_tutorial(skin: &MadSkin) {
    println!("{}", skin.inline(TUTORIAL));
}

fn prepare(is_debug: bool) -> AnyhowResult<()> {
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
// this has an optional argument of a path to write the model to
fn train(write_file: Option<&Path>, is_debug: bool) -> AnyhowResult<()> { // Check that the dataset is already present in /tmp/deepmath
    log::info!("Finding downloaded dataset");
    let default_output = &std::env::current_dir()?.join("deepmath_data/model.dat");
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

struct PredictOptions {
    is_predict_load: bool,
    is_predict_to_file: bool,
}

impl PredictOptions {
    pub fn new(is_load: bool, is_to_file: bool) -> PredictOptions {
        PredictOptions {
            is_predict_load: is_load,
            is_predict_to_file: is_to_file,
        }
    }
    pub fn is_predict_load(self) -> bool {
        self.is_predict_load
    }
    pub fn is_predict_to_file(self) -> bool {
        self.is_predict_to_file
    }
}

// Uses pre-trained model to make predictions
fn predict(
    model_to_load: Option<&Path>,
    is_debug: bool,
) -> AnyhowResult<()> {
    Ok(())
}

fn argparse() -> clap::ArgMatches {
    App::new("Deepmath")
        .version("1.0")
        .about("Deep learning model for symbolic mathematics in Rust.")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Toggles debug mode for verbose output. \
                    This MUST be run with another command line argument."),
        )
        .arg(
            Arg::new("tutorial")
                .short('t')
                .long("tutorial")
                .help("Shows the Deepmath tutorial"),
        )
        .arg(
            Arg::new("prepare")
                .short('p')
                .long("prepare")
                .help("Downloads training and test data from Facebook AI archives"),
        )
        .arg(
            Arg::new("train")
                .short('r')
                .long("train")
                .help("Train model and save model to the default location"),
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
                .short('b')
                .long("predict")
                .help(
                    "Uses trained model to predict. Shows WebView \
                    UI for interacting with trained model.",
                )
                .takes_value(true),
        )
        .arg(
            Arg::new("load")
                .short('i')
                .long("load")
                .help(
                    "Optionally load a pre-trained model \
                    from a .dat model archive. \
                    Must be used with --predict",
                )
                .takes_value(true),
        )
        .get_matches()
}

fn main() -> AnyhowResult<()> {
    // Handle empty argument
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
    } else  {
        // Uses a much-simplified set of arguments compared to
        // the original implementation
        let matches = argparse();
        simple_logger::init().unwrap();
        let is_show_tutorial = matches.is_present("tutorial");
        let is_debug_mode = matches.is_present("debug");
        let is_prepare = matches.is_present("prepare");
        let is_train = matches.is_present("train");
        let is_train_to_file = matches.is_present("train_to");
        let is_predict = matches.is_present("predict");
        let is_predict_load = matches.is_present("load");

        /* These are ordered so that even if the user gives an insane
        amount of arguments or even every possible argument at once:
          E.g.
                deepmath --tutorial --prepare --train_to "model.dat" --predict --debug
          Deepmath would still function properly, of course
          it would have to do everything, but it doesn't break
        */
        let mut is_parsed = false;
        if is_show_tutorial {
            is_parsed = true;
            let skin = MadSkin::default();
            show_tutorial(&skin);
        }
        if is_prepare {
            is_parsed = true;
            if is_debug_mode {
                prepare(true)?;
            } else {
                prepare(false)?;
            }
        }
        // regular train, to default path
        if is_train {
            is_parsed = true;
            if is_debug_mode {
                train(None, true)?;
            } else {
                log::info!("Sorry, training to file (standard) is not yet done");
            }
        }
        // train to specific path
        if is_train_to_file {
            is_parsed = true;
            if is_debug_mode {
                let model_file = Path::new(matches.value_of("train_to").unwrap());
                train(Some(model_file), true)?;
            } else {
                train(None, true)?;
            }
        }
        // predict
        if is_predict {
            is_parsed = true;
            let model_to_load = match is_predict_load {
                true => Some(Path::new(matches.value_of("load").unwrap())),
                false => unimplemented!()
            };

            if is_debug_mode {
                predict(model_to_load, true)?;
            } else {
                predict(model_to_load, false)?;
            }
        }
        if !is_parsed {
        // catch-all for everything else
            log::error!("Sorry, Deepmath couldn't figure out how you wanted to run. This might be because you didn't specify the proper command-line flags. Try running {}", "deepmath --tutorial".green());
        }
    }
    Ok(())
}
