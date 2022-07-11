use colored::{ColoredString, Colorize};
use std::fmt::{Display, Formatter, Write};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub struct Error {
  pub at: Location,
  pub msg: ErrorMessage,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}] {}", self.at, self.msg)
  }
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

#[derive(Debug, Clone)]
pub struct ErrorMessage {
  source: &'static str,
  message: String,
}

impl Display for ErrorMessage {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}] {}", self.source, self.message)
  }
}

impl From<String> for ErrorMessage {
  fn from(message: String) -> Self {
    Self {
      source: "Other",
      message,
    }
  }
}

impl From<&str> for ErrorMessage {
  fn from(message: &str) -> Self {
    message.into()
  }
}

impl From<minreq::Response> for ErrorMessage {
  fn from(req: minreq::Response) -> Self {
    Self {
      source: "Response",
      message: format!("{}: {}", req.status_code, req.reason_phrase),
    }
  }
}

impl From<minreq::Error> for ErrorMessage {
  fn from(err: minreq::Error) -> Self {
    match err {
      minreq::Error::SerdeJsonError(err) => err.into(),
      err => Self {
        source: "MinReq",
        message: err.to_string(),
      },
    }
  }
}

impl From<serde_json::Error> for ErrorMessage {
  fn from(err: serde_json::Error) -> Self {
    Self {
      source: "SerdeJson",
      message: err.to_string(),
    }
  }
}

impl From<(&str, serde_json::Error)> for ErrorMessage {
  fn from((json, err): (&str, serde_json::Error)) -> Self {
    Self {
      source: "SerdeJson",
      message: format!("{err}: \n{json}"),
    }
  }
}

impl From<toml::de::Error> for ErrorMessage {
  fn from(err: toml::de::Error) -> Self {
    Self {
      source: "Toml",
      message: err.to_string(),
    }
  }
}

impl From<(&str, toml::de::Error)> for ErrorMessage {
  fn from((toml, err): (&str, toml::de::Error)) -> Self {
    Self {
      source: "Toml",
      message: format!("{err}: \n{toml}"),
    }
  }
}

impl From<std::io::Error> for ErrorMessage {
  fn from(err: std::io::Error) -> Self {
    Self {
      source: "IO",
      message: err.to_string(),
    }
  }
}
