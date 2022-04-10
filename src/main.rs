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

#[cfg(debug_assertions)]
const DEFAULT_LEVEL_FILTER: LevelFilter = LevelFilter::Debug;
#[cfg(not(debug_assertions))]
const DEFAULT_LEVEL_FILTER: LevelFilter = LevelFilter::Info;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  /// Path to the packwiz directory containing "pack.toml"
  #[clap(long, short = 'p', default_value = "./", value_hint = clap::ValueHint::DirPath)]
  path: PathBuf,
  /// Path to the directory contains all the mod metadata files
  #[clap(long, short = 'm', default_value = "mods", value_hint = clap::ValueHint::DirPath)]
  mods: PathBuf,
  /// Should mod path be it's own path instead of being relative to --path
  #[clap(long, short = 'M')]
  mods_custom: bool,
  /// Output file
  #[clap(long, short = 'o', default_value = "modlist.md")]
  output: PathBuf,
  #[clap(long, short = 'O')]
  output_custom: bool,
  /// Overwrites output if it already exists
  #[clap(long, short = 'F')]
  force: bool,
  /// Debug log (trace > debug > silent)
  #[clap(long, short = 'd')]
  debug: bool,
  /// Trace log (trace > debug > silent)
  #[clap(long, short = 't')]
  trace: bool,
  /// No log (trace > debug > silent)
  #[clap(long, short = 's')]
  silent: bool,
  /// Format
  #[clap(long, short = 'f', default_value = "[{NAME}]({URL}) - {DESCRIPTION}\n")]
  format: String,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let level = match () {
    _ if args.trace => LevelFilter::Trace,
    _ if args.debug => LevelFilter::Debug,
    _ if args.silent => LevelFilter::Off,
    _ => DEFAULT_LEVEL_FILTER,
  };

  SimpleLogger::new().with_level(level).init().unwrap();

  let result = generate(&args).await;

  if let Err(err) = result {
    error_handler(err);
  }
}
