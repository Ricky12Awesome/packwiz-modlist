use std::error::Error;
use std::path::PathBuf;

use log::error;
use thiserror::Error;

pub type GlobalResult<T> = Result<T, GlobalError>;

#[derive(Debug, Error)]
pub enum ValidationError {
  #[error("{0} does not exist")]
  DirNotExist(PathBuf),
  #[error("{0} must be a directory")]
  MustBeDir(PathBuf),
  #[error("pack.toml was not found in {0}")]
  PackNotFound(PathBuf),
}

#[derive(Debug, Error)]
#[error("")]
pub enum GlobalError {
  Validation(#[from] ValidationError),
  FileIO(#[from] std::io::Error),
  TomlDeserialize(#[from] toml::de::Error),
  JsonDeserialize(#[from] serde_json::Error),
  Clap(#[from] clap::Error),
  Reqwest(#[from] reqwest::Error),
  Unknown(#[from] Box<dyn Error>),
}

pub fn error_handler(err: GlobalError) {
  match err {
    GlobalError::Validation(err) => error!("Validation: {err}"),
    GlobalError::FileIO(err) => error!("File: {err}"),
    GlobalError::TomlDeserialize(err) => error!("Toml: {err}"),
    GlobalError::JsonDeserialize(err) => error!("Json: {err}"),
    GlobalError::Clap(err) => error!("Clap: {err}"),
    GlobalError::Reqwest(err) => error!("Reqwest: {err}"),
    GlobalError::Unknown(err) => error!("Unknown: {err}"),
  }
}