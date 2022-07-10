#![allow(unused)] // ignore warning

use crate::request::curseforge::get_curseforge_mods;
use crate::request::modrinth::get_modrinth_projects;
use log::Log;

mod consts;
pub mod error;
mod request;

fn setup_logging() {
  simple_logger::SimpleLogger::new()
    .without_timestamps()
    .env()
    .init()
    .unwrap();

  colored::control::set_override(true);

  #[cfg(windows)]
  colored::control::set_virtual_terminal(true);
}

fn main() {
  setup_logging();

  // let projects = get_modrinth_projects(&["AANobbMI"]);
  let projects = get_curseforge_mods(&[238222]);

  println!("{projects:#?}");
}
