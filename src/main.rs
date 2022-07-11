#![allow(unused)] // ignore warning

use crate::cache::Cache;
use crate::error::Error;
use crate::parser::packwiz::PackwizParser;
use crate::request::curseforge::get_curseforge_mods;
use crate::request::modrinth::get_modrinth_projects;
use crate::request::Mod;
use itertools::Itertools;
use log::Log;
use std::collections::HashMap;
use std::ops::Index;

mod app;
mod args;
mod cache;
mod consts;
mod error;
mod macros;
mod parser;
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

fn run() -> Result<(), Error> {
  // let mut cache = Cache::load(".packwizml.cache.json")?;
  //
  // // Sodium
  // let mr_projects = get_modrinth_projects(&["AANobbMI"])?;
  //
  // // JEI
  // let cf_projects = get_curseforge_mods(&[238222])?;
  //
  // let mr_project: Mod = mr_projects[0].clone().into();
  // let cf_project: Mod = cf_projects[0].clone().into();
  //
  // let mods = vec![mr_project, cf_project];
  //
  // cache.save()?;

  PackwizParser::load_from(r"C:\Users\ricky\dev\modpacks\OptiCraft\mod")?;

  Ok(())
}

fn main() {
  setup_logging();

  if let Err(err) = run() {
    log::error!("{err}");
  }
}
