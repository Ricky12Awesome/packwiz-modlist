use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;

use crate::error::{error_handler, GlobalError, GlobalResult, ValidationError};
use crate::output::generate;

mod data;
mod error;
mod misc;
mod object;
mod output;

const LOG_VALUES: [&str; 6] = ["Off", "Error", "Warn", "Info", "Debug", "Trace"];

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
  /// Set Log level
  #[clap(long, short = 'v', ignore_case = true, default_value = "Warn", possible_values = LOG_VALUES)]
  log_level: LevelFilter,
  /// Prints about this program
  #[clap(long, global = true)]
  about: bool,
  /// Specify a custom format
  #[clap(long, short = 'f', default_value = "[{NAME}]({URL}) - {DESCRIPTION}\n")]
  format: String,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();
  colored::control::set_override(true);
  #[cfg(windows)]
  colored::control::set_virtual_terminal(true).unwrap();

  if args.about {
    println!("E");
    return;
  }

  SimpleLogger::new().with_level(args.log_level).init().unwrap();

  let result = generate(&args).await;

  if let Err(err) = result {
    error_handler(err);
  }
}
