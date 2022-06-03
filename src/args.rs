use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use log::LevelFilter;

const LOG_VALUES: [&str; 6] = ["Off", "Error", "Warn", "Info", "Debug", "Trace"];
const COLOR_MODES: [&str; 3] = ["Auto", "Always", "Never"];

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  /// Path to the packwiz directory containing 'pack.toml'
  #[clap(long, short = 'p', default_value = "./", value_hint = clap::ValueHint::DirPath)]
  pub path: PathBuf,
  /// Path to the directory contains all the mod metadata files
  #[clap(long, short = 'm', default_value = "mods", value_hint = clap::ValueHint::DirPath)]
  pub mods: PathBuf,
  /// Disable '--mods' being relative to '--path'
  #[clap(short = 'M')]
  pub mods_custom: bool,
  /// Specify a output file
  #[clap(long, short = 'o', default_value = "modlist.md")]
  pub output: PathBuf,
  /// Disable'`--output' being relative to '--path'
  #[clap(short = 'O')]
  pub output_custom: bool,
  /// Overwrites output if it already exists
  #[clap(long, short = 'F')]
  pub force: bool,
  /// Sets the verbosity of logging
  #[clap(long, short = 'v', ignore_case = true, default_value = "Warn", possible_values = LOG_VALUES)]
  pub log_level: LevelFilter,
  #[clap(long, short = 'c', ignore_case = true, default_value = "Auto", possible_values = COLOR_MODES)]
  pub color_mode: ColorMode,
  /// Prints about this program
  #[clap(long, global = true)]
  pub about: bool,
  /// Prints json output
  #[clap(long, global = true)]
  pub json: bool,
  /// Specify a custom format
  #[clap(long, short = 'f', default_value = "[{NAME}]({URL}) - {DESCRIPTION}\n")]
  pub format: String,
}

#[derive(Debug, Copy, Clone)]
pub enum ColorMode {
  Auto,
  Always,
  Never,
}

impl FromStr for ColorMode {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "auto" => Ok(ColorMode::Auto),
      "always" => Ok(ColorMode::Always),
      "never" => Ok(ColorMode::Never),
      _ => Err(format!("'{s}' is not a valid color mode"))
    }
  }
}