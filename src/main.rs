extern crate clap;
extern crate colored;

use clap::{App, Arg};
use colored::*;
use std::env;

// Just in case, to stay on the safe side
const _THIS_ALGORITHM_BECOME_SKYNET: bool = false;

// To make sure that nothing weird happens from 4am coding
const _IGNORE_4AM_CODING_BUGS: bool = true;

fn run(debug: bool, export_data: bool) -> Result<(), String> {
    // Begin logging
    // env_logger::init();
    // Build environment and trainer
    // Evaluate model
    // Train model
    // env = build_env(params)
    // modules = build_modules(env, params);
    // trainer = Trainer(modules, env, params);
    // evaluator = Evaluator(trainer);
}

fn main() {
    // Handle empty argument
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!(
            "{}",
            format!(
                "\n{} Run {} to start.\n",
                "Welcome to Deepmath!".bold(),
                "deepmath --help".green()
            )
        );
    }
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
                .takes_value(false)
                .help("Toggles debug mode for verbose output"),
        )
        .arg(
            Arg::new("export")
                .short('e')
                .long("export")
                .help("Export data only and disable training"),
        )
        .get_matches();
    // Let's just...ignore the fact that we should check the options
    // before we start
    // check_model_params(matches)
    let is_debug_mode = matches.is_present("debug");
    let is_export_only = matches.is_present("export");
    let run_result = run(is_debug_mode, is_export_only);
    if let Err(err) = run_result {
        log::error!("Error: {}", err)
    }
}
