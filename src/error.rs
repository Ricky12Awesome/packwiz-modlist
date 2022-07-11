use colored::{ColoredString, Colorize};
use std::fmt::{Display, Formatter, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("[{at}] {kind}")]
pub struct Error {
  pub at: Location,
  pub kind: ErrorKind,
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
pub enum ErrorKind {
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
  Other(String),
}

impl From<String> for ErrorKind {
  fn from(message: String) -> Self {
    Self::Other(message)
  }
}

impl From<&str> for ErrorKind {
  fn from(message: &str) -> Self {
    message.into()
  }
}

impl From<minreq::Response> for ErrorKind {
  fn from(req: minreq::Response) -> Self {
    Self::Response(req.status_code, req.reason_phrase)
  }
}

impl From<minreq::Error> for ErrorKind {
  fn from(err: minreq::Error) -> Self {
    match err {
      minreq::Error::SerdeJsonError(err) => err.into(),
      err => Self::MinReq(err),
    }
  }
}

impl From<serde_json::Error> for ErrorKind {
  fn from(err: serde_json::Error) -> Self {
    Self::Json("[No Json Provided]".into(), err)
  }
}

impl From<(&str, serde_json::Error)> for ErrorKind {
  fn from((json, err): (&str, serde_json::Error)) -> Self {
    Self::Json(json.into(), err)
  }
}

impl From<toml::de::Error> for ErrorKind {
  fn from(err: toml::de::Error) -> Self {
    Self::Toml("[No Toml Provided]".into(), err)
  }
}

impl From<(&str, toml::de::Error)> for ErrorKind {
  fn from((toml, err): (&str, toml::de::Error)) -> Self {
    Self::Toml(toml.into(), err)
  }
}

impl From<(PathBuf, std::io::Error)> for ErrorKind {
  fn from((path, err): (PathBuf, std::io::Error)) -> Self {
    Self::FileIo(path, err)
  }
}
