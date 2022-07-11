#[macro_export]
macro_rules! location {
  () => {
    $crate::error::Location {
      file: file!(),
      line: line!(),
      col: column!(),
    }
  };
}

#[macro_export]
macro_rules! error {
  () => {
    |err| $crate::error!(err)
  };
  ($msg:expr) => {
    $crate::error::Error {
      at: $crate::location!(),
      kind: $msg.into(),
    }
  };
  (from $var:tt($($arg:expr),*)) => {
    $crate::error::Error {
      at: $crate::location!(),
      kind: $crate::error::ErrorKind::$var($($arg.into()),*),
    }
  };
}