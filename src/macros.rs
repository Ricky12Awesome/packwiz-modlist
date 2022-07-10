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
      msg: $msg.into(),
    }
  };
}

#[macro_export]
macro_rules! request_returns {
  ($r:expr) => {
    match $r.status_code {
      200 => $r.json().map_err($crate::error!()),
      _ => Err($crate::error!($r)),
    }
  };

  ($r:expr, $T:tt) => {
    match $r.status_code {
      200 => $r.json::<$T>().map_err($crate::error!()),
      _ => Err($crate::error!($r)),
    }
  };
}
