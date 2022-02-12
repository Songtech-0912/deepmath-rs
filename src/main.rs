#![allow(dead_code)]
use anyhow::Result as AnyhowResult;
use clap::{App, Arg};
use colored::*;
use data_retrieve::checks;
use data_retrieve::download;
use data_retrieve::unarchive;
use deepmath::data_retrieve;
use deepmath::model;
use std::env;
// use std::fs::File;
// use std::io::prelude::*;
use crossterm::style::Color::*;
use std::path::Path;
use termimad::*;

// For sake of time I will be hardcoding some properties
const MAX_EPOCHS: i64 = 30000;

// Just in case, to stay on the safe side
const _THIS_ALGORITHM_BECOME_SKYNET: bool = false;

// To make sure that nothing weird happens from 4am coding
const _IGNORE_4AM_CODING_BUGS: bool = true;

// Deepmath tutorial
const TUTORIAL: &str = include_str!("./include/tutorial.md");

fn show_tutorial(skin: &MadSkin) {
    skin.print_text(TUTORIAL);
}

fn show_welcome() {
    println!(
        "{}",
        format!(
            "\n{} Run {} to start.\n",
            "Welcome to Deepmath!".bold(),
            "deepmath --tutorial".green()
        )
    );
}

// Downloads and prepares dataset
fn prepare(is_debug: bool) -> AnyhowResult<()> {
    // Check if data has been downloaded
    if !checks::data_present()? {
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
    } else {
        log::error!(
            "You've already downloaded the data, run {} instead!",
            "deepmath --train".green()
        );
        Ok(())
    }
}

// Creates and trains model
// this has an optional argument of a path to write the model to
fn train(write_file: Option<&Path>, is_debug: bool) -> AnyhowResult<()> {
    // If model is already trained
    let trained_model = model::utils::model_location()?;
    if trained_model.exists() {
        log::error!(
            "You've already trained the model, run {} instead!",
            "deepmath --predict".green()
        );
        Ok(())
    } else {
        // Check that the dataset is already present in $PWD/deepmath
        log::info!("Finding downloaded dataset");
        let default_output = model::utils::model_location()?;
        let output_file = write_file.unwrap_or(&default_output);
        let mode = match is_debug {
            true => "debug",
            false => "standard",
        };
        log::info!("Training under {} mode to {}", mode, &output_file.display());
        if checks::data_present()? {
            if is_debug {
                log::info!("Preparing model")
            }
            // Do the actual machine learning stuff
            // build model, train model, etc.
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
}

// Uses pre-trained model to make predictions
fn predict(model_to_load: Option<&Path>, is_debug: bool) -> AnyhowResult<()> {
    let default_model_path = model::utils::model_location()?;
    let model_path = model_to_load.unwrap_or(&default_model_path);
    if model_path.exists() {
        log::info!(
            "Found model at {}",
            model_to_load.unwrap().to_str().unwrap()
        );
        if is_debug {
            log::info!("Placeholder debug message for predict...");
        }
        log::info!("Loading model...")
        // Check if model loading is successful
        // If model loading is success then start server
        // Then start WebView UI
        // Then do the predicting stuff and show it on the UI...
    } else {
        log::error!(
            "Model wasn't found. Did you forget to run {}?",
            "deepmath --prepare --train".green()
        )
    }
    Ok(())
}

fn argparse() -> clap::ArgMatches {
    App::new("Deepmath")
        .version("1.0")
        .about("Deep learning model for symbolic mathematics in Rust.")
        .arg(Arg::new("debug").short('d').long("debug").help(
            "Toggles debug mode for verbose output. \
                    This MUST be run with another command line argument.",
        ))
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
                .help("Train model and save trained model to the default location"),
        )
        .arg(
            Arg::new("train_to")
                .short('e')
                .long("train_to")
                .help("Train model and save trained model to a specified location")
                .takes_value(true),
        )
        .arg(Arg::new("predict").short('b').long("predict").help(
            "Uses trained model to predict. Shows WebView \
                    UI for interacting with trained model.",
        ))
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
        show_welcome();
    } else {
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
            let mut skin = MadSkin::default();
            skin.set_headers_fg(AnsiValue(178));
            skin.bold.set_fg(Yellow);
            skin.italic.set_fg(Magenta);
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
        if is_train && !is_train_to_file {
            is_parsed = true;
            if is_debug_mode {
                train(None, true)?;
            } else {
                train(None, false)?;
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
                false => None,
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
