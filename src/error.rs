use colored::{ColoredString, Colorize};
use std::fmt::{Display, Formatter, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("[{at}] {msg}")]
pub struct Error {
  pub at: Location,
  pub msg: ErrorMessage,
}

#[derive(Debug, Clone)]
pub struct Location {
  pub file: &'static str,
  pub line: u32,
  pub col: u32,
}

impl Display for Location {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}:{}", self.file, self.line, self.col)
  }
}

#[derive(Error, Debug)]
#[error("{}")]
pub enum ErrorMessage {
  #[error("{0}: {1}")]
  FileIo(PathBuf, std::io::Error),
  #[error("{0}")]
  Io(#[from] std::io::Error),
  #[error("{0}: {1}")]
  Json(String, serde_json::Error),
  #[error("{0}: {1}")]
  Toml(String, toml::de::Error),
  #[error("{0}: {1}")]
  Response(i32, String),
  #[error("{0}")]
  MinReq(minreq::Error),
  #[error("{0}")]
  Other(String),
}

impl From<String> for ErrorMessage {
  fn from(message: String) -> Self {
    Self::Other(message)
  }
}

impl From<&str> for ErrorMessage {
  fn from(message: &str) -> Self {
    message.into()
  }
}

impl From<minreq::Response> for ErrorMessage {
  fn from(req: minreq::Response) -> Self {
    Self::Response(req.status_code, req.reason_phrase)
  }
}

impl From<minreq::Error> for ErrorMessage {
  fn from(err: minreq::Error) -> Self {
    match err {
      minreq::Error::SerdeJsonError(err) => err.into(),
      err => Self::MinReq(err),
    }
  }
}

impl From<serde_json::Error> for ErrorMessage {
  fn from(err: serde_json::Error) -> Self {
    Self::Json("[No Json Provided]".into(), err)
  }
}

impl From<(&str, serde_json::Error)> for ErrorMessage {
  fn from((json, err): (&str, serde_json::Error)) -> Self {
    Self::Json(json.into(), err)
  }
}

impl From<toml::de::Error> for ErrorMessage {
  fn from(err: toml::de::Error) -> Self {
    Self::Toml("[No Toml Provided]".into(), err)
  }
}

impl From<(&str, toml::de::Error)> for ErrorMessage {
  fn from((toml, err): (&str, toml::de::Error)) -> Self {
    Self::Toml(toml.into(), err)
  }
}
