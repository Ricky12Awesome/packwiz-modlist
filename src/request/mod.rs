use crate::consts::USER_AGENT;
use minreq::{Request, URL};

pub mod curseforge;
pub mod modrinth;

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

pub fn get<T: Into<URL>>(url: T) -> Request {
  minreq::get(url)
    .with_header("User-Agent", USER_AGENT)
    .with_header("Content-Type", "application/json")
}

pub fn post<T: Into<URL>>(url: T) -> Request {
  minreq::post(url)
    .with_header("User-Agent", USER_AGENT)
    .with_header("Content-Type", "application/json")
}
