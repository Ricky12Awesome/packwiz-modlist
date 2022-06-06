use std::convert::Infallible;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use log::LevelFilter;

const LOG_VALUES: [&str; 6] = ["Off", "Error", "Warn", "Info", "Debug", "Trace"];
const COLOR_MODES: [&str; 3] = ["Auto", "Always", "Never"];
const SORTING_MODES: [&str; 5] = ["Name", "Title", "Slug", "Id", "None"];

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  /// Path to the packwiz directory containing 'pack.toml'
  #[clap(long, short = 'p', default_value = "./", value_hint = clap::ValueHint::DirPath)]
  pub path: PathBuf,
  /// Set the cache file
  #[clap(long, default_value = "./.packwizml.cache")]
  pub cache: PathBuf,
  /// Path to the directory contains all the mod metadata files
  #[clap(long, short = 'm', default_value = "mods", value_hint = clap::ValueHint::DirPath)]
  pub mods: PathBuf,
  /// Disable '--mods' being relative to '--path'
  #[clap(short = 'M')]
  pub mods_custom: bool,
  /// Set an output file
  #[clap(long, short = 'o')]
  pub output: Option<PathBuf>,
  /// Disable'`--output' being relative to '--path'
  #[clap(short = 'O')]
  pub output_custom: bool,
  /// Overwrites output if it already exists
  #[clap(long, short = 'F')]
  pub force: bool,
  /// Sets the verbosity of logging
  #[clap(long, short = 'v', ignore_case = true, default_value = "Warn", possible_values = LOG_VALUES)]
  pub log_level: LevelFilter,
  /// Sets the color mode
  #[clap(long, short = 'c', ignore_case = true, default_value = "Auto", possible_values = COLOR_MODES)]
  pub color_mode: ColorMode,
  /// Sets the sorting mode
  #[clap(long, short = 's', ignore_case = true, default_value = "None", possible_values = SORTING_MODES)]
  pub sort_by: SortingMode,
  /// Sets if sorting should be reverse
  #[clap(long, short = 'r')]
  pub reverse: bool,
  /// Prints about this program
  #[clap(long, global = true)]
  pub about: bool,
  /// Prints json output
  #[clap(long, global = true)]
  pub json: bool,
  /// Set a custom format
  #[clap(
    long,
    short = 'f',
    allow_hyphen_values = true,
    default_value = "- [{NAME}]({URL}) - {DESCRIPTION}\n"
  )]
  pub format: String,
}

#[derive(Debug, Copy, Clone)]
pub enum ColorMode {
  Auto,
  Always,
  Never,
}

impl FromStr for ColorMode {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "auto" => Ok(Self::Auto),
      "always" => Ok(Self::Always),
      "never" => Ok(Self::Never),
      _ => unreachable!(),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub enum SortingMode {
  Name,
  Title,
  Slug,
  Id,
  None,
}

impl FromStr for SortingMode {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "name" => Ok(Self::Name),
      "title" => Ok(Self::Title),
      "slug" => Ok(Self::Slug),
      "id" => Ok(Self::Id),
      "none" => Ok(Self::None),
      _ => unreachable!(),
    }
  }
}
