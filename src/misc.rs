use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use serde::de::DeserializeOwned;

use crate::error::{GlobalError, GlobalErrorCustom, GlobalResult};

pub fn read_toml_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> GlobalResult<T> {
  let file = File::open(path)?;
  let bytes = file.bytes().collect::<Result<Vec<_>, _>>()?;

  toml::from_slice::<T>(&bytes).map_err(GlobalError::from)
}

#[derive(Debug, Copy, Clone)]
pub enum ColorMode {
  Auto,
  Always,
  Never,
}

impl FromStr for ColorMode {
  type Err = GlobalErrorCustom;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "auto" => Ok(ColorMode::Auto),
      "always" => Ok(ColorMode::Always),
      "never" => Ok(ColorMode::Never),
      _ => Err(GlobalErrorCustom::new("ColorMode", format!("'{s}' is not a valid color mode")))
    }
  }
}