use std::error::Error;

use log::error;
use thiserror::Error;

use crate::data::ValidationError;

pub type GlobalResult<T> = Result<T, GlobalError>;

#[derive(Debug, Error)]
#[error("")]
pub enum GlobalError {
  Validation(#[from] ValidationError),
  FileIO(#[from] std::io::Error),
  TomlDeserialize(#[from] toml::de::Error),
  JsonDeserialize(#[from] serde_json::Error),
  Clap(#[from] clap::Error),
  Unknown(#[from] Box<dyn Error>),
}

impl GlobalError {
  fn unknown(err: impl Error + 'static) -> Self {
    Self::Unknown(Box::new(err))
  }
}

pub fn error_handler(err: GlobalError) {
  match err {
    GlobalError::Validation(err) => error!("Validation: {err}"),
    GlobalError::FileIO(err) => error!("File: {err}"),
    GlobalError::TomlDeserialize(err) => error!("Toml: {err}"),
    GlobalError::JsonDeserialize(err) => error!("Json: {err}"),
    GlobalError::Clap(err) => error!("Clap: {err}"),
    GlobalError::Unknown(err) => error!("Unknown: {err}")
  }
}