use clap::{App, Arg};
use std::env;
use colored::*;
use termimad::*;
use anyhow::Result as AnyhowResult;
use deepmath::data_retrieve::download as download;
use deepmath::data_retrieve::unarchive as unarchive;

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
deepmath --prepare
```

**Step 2: Train the model**
```
deepmath --train_to "model.dat"
```

**Step 3: Use the model to solve**
```
./target/release/deepmath --load "model.dat" --input "equations.yml" --predict
```

Note that you can choose to not specify the `--input` option, if that
is the case, then deepmath will solve a default set of equations.
"#;

fn show_tutorial(skin: &MadSkin) {
    println!("{}", skin.inline(TUTORIAL));
}


fn run(is_debug: bool) -> AnyhowResult<()> {
    simple_logger::init().unwrap();
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

fn main() {
    // Handle empty argument
    let mut is_run = false;
    let args: Vec<String> = env::args().collect();
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
                    .takes_value(false)
                    .help("Toggles debug mode for verbose output"),
            )
            .arg(
                Arg::new("export")
                    .short('e')
                    .long("export")
                    .help("Export data only and disable training"),
            )
            .arg(
                Arg::new("tutorial")
                    .short('t')
                    .long("tutorial")
                    .help("Shows the Deepmath tutorial"),
            )
            .get_matches();
        // Let's just...ignore the fact that we should check the options
        // before we start
        // check_model_params(matches)
        let is_show_tutorial = matches.is_present("tutorial");
        // Single out is_show_tutorial because it doesn't require
        // executing the run() function in any way
        if is_show_tutorial {
            let skin = MadSkin::default();
            show_tutorial(&skin);
        } else {
            let is_debug_mode = matches.is_present("debug");
            // let is_export_only = matches.is_present("export");
            let run_result = run(is_debug_mode);
            if let Err(err) = run_result {
                log::error!("Error: {}", err)
            }
        }
    }
}
