#![allow(unused)] // ignore warning

use std::collections::HashMap;
use std::ops::Index;
use itertools::Itertools;
use crate::request::curseforge::get_curseforge_mods;
use crate::request::modrinth::get_modrinth_projects;
use crate::request::Mod;
use log::Log;
use crate::cache::Cache;

mod cache;
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

  let mut cache = Cache::load(".packwizml.cache.json").unwrap();

  // Sodium
  let mr_projects = get_modrinth_projects(&["AANobbMI"]).unwrap();

  // JEI
  let cf_projects = get_curseforge_mods(&[238222]).unwrap();

  let mr_project: Mod = mr_projects[0].clone().into();
  let cf_project: Mod = cf_projects[0].clone().into();

  cache.save();

  let mods = vec![mr_project, cf_project]
    .into_iter()
    .into_group_map_by(|m| m.id.clone())
    .into_iter()
    .map(|l| (l.0, l.1.first().unwrap().clone()))
    .collect::<HashMap<_, _>>();

  cache.write_all(mods);
  cache.save();
}
