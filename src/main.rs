#![allow(unused)] // ignore warning

use crate::request::curseforge::get_curseforge_mods;
use crate::request::modrinth::get_modrinth_projects;
use crate::request::Project;
use log::Log;

mod consts;
mod error;
mod macros;
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

  // Sodium
  let mr_projects = get_modrinth_projects(&["AANobbMI"]).unwrap();

  // JEI
  let cf_projects = get_curseforge_mods(&[238222]).unwrap();

  let mr_project: Project = mr_projects[0].clone().into();
  let cf_project: Project = cf_projects[0].clone().into();

  let mods = vec![mr_project, cf_project];

  println!("{mods:#?}");
}
