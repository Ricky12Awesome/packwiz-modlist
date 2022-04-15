use std::error::Error;
use std::path::PathBuf;

use log::error;
use thiserror::Error;

pub type GlobalResult<T> = Result<T, GlobalError>;

#[derive(Debug, Error)]
pub enum ValidationError {
  #[error("{0} already exists, to overwrite run command again with '--force' or '-F'")]
  OutputAlreadyExits(PathBuf),
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
  Custom(#[from] GlobalErrorCustom),
  Unknown(#[from] Box<dyn Error>),
}

#[derive(Debug, Error)]
#[error("{typ}: {msg}")]
pub struct GlobalErrorCustom {
  typ: String,
  msg: String,
}

impl GlobalErrorCustom {
  pub fn new(typ: impl ToString, msg: impl ToString) -> Self {
    Self {
      typ: typ.to_string(),
      msg: msg.to_string(),
    }
  }
}

impl GlobalError {
  pub fn custom(typ: impl ToString, msg: impl ToString) -> Self {
    GlobalErrorCustom::new(typ, msg).into()
  }
}

pub fn error_handler(err: GlobalError) {
  match err {
    GlobalError::Validation(err) => error!("Validation: {err}"),
    GlobalError::FileIO(err) => error!("File: {err}"),
    GlobalError::TomlDeserialize(err) => error!("Toml: {err}"),
    GlobalError::JsonDeserialize(err) => error!("Json: {err}"),
    GlobalError::Clap(err) => error!("Clap: {err}"),
    GlobalError::Reqwest(err) => error!("Reqwest: {err}"),
    GlobalError::Custom(err) => error!("{err}"),
    GlobalError::Unknown(err) => error!("Unknown: {err}"),
  }
}