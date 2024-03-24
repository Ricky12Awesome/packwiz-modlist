use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use minreq::Response;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{}")]
pub enum Error {
  #[error("{1} (\"{0}\")")]
  FileIo(PathBuf, std::io::Error),
  #[error("{0}")]
  Io(#[from] std::io::Error),
  #[error("{1}: \n{0}")]
  Json(String, serde_json::Error),
  #[error("{1}: \n{0}")]
  Toml(String, toml::de::Error),
  #[error("{0}: {1}")]
  Response(i32, String),
  #[error("{0}")]
  MinReq(minreq::Error),
  #[error("{0}")]
  TextParser(#[from] crate::parser::text::ParseError),
  #[error("{0}")]
  Other(String),
}

impl From<String> for Error {
  fn from(message: String) -> Self {
    Self::Other(message)
  }
}

impl From<&str> for Error {
  fn from(message: &str) -> Self {
    message.into()
  }
}

impl From<minreq::Response> for Error {
  fn from(req: minreq::Response) -> Self {
    Self::Response(req.status_code, req.reason_phrase)
  }
}

impl From<minreq::Error> for Error {
  fn from(err: minreq::Error) -> Self {
    match err {
      minreq::Error::SerdeJsonError(err) => err.into(),
      err => Self::MinReq(err),
    }
  }
}

impl From<(&str, minreq::Error)> for Error {
  fn from((res, err): (&str, minreq::Error)) -> Self {
    match err {
      minreq::Error::SerdeJsonError(err) => (res, err).into(),
      err => Self::MinReq(err),
    }
  }
}

impl From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Self {
    Self::Json("[No Json Provided]".into(), err)
  }
}

impl From<(&str, serde_json::Error)> for Error {
  fn from((json, err): (&str, serde_json::Error)) -> Self {
    Self::Json(json.into(), err)
  }
}

impl From<toml::de::Error> for Error {
  fn from(err: toml::de::Error) -> Self {
    Self::Toml("[No Toml Provided]".into(), err)
  }
}

impl From<(&str, toml::de::Error)> for Error {
  fn from((toml, err): (&str, toml::de::Error)) -> Self {
    Self::Toml(toml.into(), err)
  }
}

impl From<(PathBuf, std::io::Error)> for Error {
  fn from((path, err): (PathBuf, std::io::Error)) -> Self {
    Self::FileIo(path, err)
  }
}
