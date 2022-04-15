use std::fmt::Display;
use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;
use log::LevelFilter;
use simple_logger::SimpleLogger;

use crate::error::{GlobalError, GlobalResult, handle_error, ValidationError};
use crate::misc::ColorMode;
use crate::output::{generate, write_to_file};

mod data;
mod error;
mod misc;
mod object;
mod output;

const LOG_VALUES: [&str; 6] = ["Off", "Error", "Warn", "Info", "Debug", "Trace"];
const COLOR_MODES: [&str; 3] = ["Auto", "Always", "Never"];

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  /// Path to the packwiz directory containing 'pack.toml'
  #[clap(long, short = 'p', default_value = "./", value_hint = clap::ValueHint::DirPath)]
  path: PathBuf,
  /// Path to the directory contains all the mod metadata files
  #[clap(long, short = 'm', default_value = "mods", value_hint = clap::ValueHint::DirPath)]
  mods: PathBuf,
  /// Disable '--mods' being relative to '--path'
  #[clap(short = 'M')]
  mods_custom: bool,
  /// Specify a output file
  #[clap(long, short = 'o', default_value = "modlist.md")]
  output: PathBuf,
  /// Disable'`--output' being relative to '--path'
  #[clap(short = 'O')]
  output_custom: bool,
  /// Overwrites output if it already exists
  #[clap(long, short = 'F')]
  force: bool,
  /// Sets the verbosity of logging
  #[clap(long, short = 'v', ignore_case = true, default_value = "Warn", possible_values = LOG_VALUES)]
  log_level: LevelFilter,
  #[clap(long, short = 'c', ignore_case = true, default_value = "Auto", possible_values = COLOR_MODES)]
  color_mode: ColorMode,
  /// Prints about this program
  #[clap(long, global = true)]
  about: bool,
  /// Prints json output
  #[clap(long, global = true)]
  json: bool,
  /// Specify a custom format
  #[clap(long, short = 'f', default_value = "[{NAME}]({URL}) - {DESCRIPTION}\n")]
  format: String,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  match args.color_mode {
    ColorMode::Auto => (),
    ColorMode::Always => colored::control::set_override(true),
    ColorMode::Never => colored::control::set_override(false),
  }

  #[cfg(windows)]
  colored::control::set_virtual_terminal(true).unwrap();

  SimpleLogger::new().with_level(args.log_level).init().unwrap();

  if args.about {
    fn about(k: &str, v: impl Display) {
      println!("{}{}{}", k.bright_purple(), ": ".white(), v);
    }

    about("Name", env!("CARGO_PKG_NAME").bright_yellow());
    about("Version", env!("CARGO_PKG_VERSION").bright_red());
    about("Author", env!("CARGO_PKG_AUTHORS").bright_yellow());
    about("Description", env!("CARGO_PKG_DESCRIPTION").bright_yellow());
    about("License", env!("CARGO_PKG_LICENSE").bright_cyan());
    about("Repository", env!("CARGO_PKG_REPOSITORY").bright_blue());


    return;
  }

  let result = run(&args).await;

  if let Err(err) = result {
    handle_error(&err);
  }
}

async fn run(args: &Args) -> GlobalResult<()> {
  let data = generate(args).await?;

  if args.json {
    println!("{}", serde_json::to_string_pretty(&data).unwrap());

    return Ok(());
  }

  write_to_file(args, &data).await
}

