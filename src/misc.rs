use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::error::{GlobalError, GlobalResult};

pub fn read_toml_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> GlobalResult<T> {
  let file = File::open(path)?;
  let bytes = file.bytes().collect::<Result<Vec<_>, _>>()?;

  toml::from_slice::<T>(&bytes).map_err(GlobalError::from)
}
