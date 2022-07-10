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
    Display::fmt(&"[".bright_cyan(), f)?;
    Display::fmt(&self.at, f)?;
    Display::fmt(&"]".bright_cyan(), f)?;
    f.write_str(" ")?;
    Display::fmt(&self.msg, f)?;

    Ok(())
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
    Display::fmt(&self.file.bright_purple(), f)?;
    Display::fmt(&":".bright_cyan(), f)?;
    Display::fmt(&self.line.to_string().bright_magenta(), f)?;
    Display::fmt(&":".bright_cyan(), f)?;
    Display::fmt(&self.col.to_string().bright_magenta(), f)?;

    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct ErrorMessage {
  source: &'static str,
  message: String,
}

impl Display for ErrorMessage {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Display::fmt(&"[".bright_cyan(), f)?;
    Display::fmt(&self.source.bright_yellow(), f)?;
    Display::fmt(&"]".bright_cyan(), f)?;
    f.write_str(" ")?;
    Display::fmt(&self.message, f)?;

    Ok(())
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
