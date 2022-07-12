#![allow(unused)]

use crate::app::App;
use crate::cache::Cache;
use crate::error::Error;
use crate::parser::packwiz::PackwizParser;
use crate::parser::Parser;
use crate::parser::text::TextParser;
use crate::request::curseforge::get_curseforge_mods;
use crate::request::modrinth::get_modrinth_projects;
use crate::request::Mod;

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
  colored::control::set_virtual_terminal(true).unwrap();
}

fn run() -> Result<(), Error> {

  let cache = Cache::load(".packwiz-modlist.cache.json")?;
  // Sodium: AANobbMI (MR)
  // JEI: 238222 (CF)
  let parser = TextParser::new("mr:AANobbMI:cache_id_here\ncf:238222:cache_id_here")?;
  let app = App::new(cache, parser);

  if let Err(err) = app.run() {
    log::error!("{err}");
  }

  if let Err(err) = app.close() {
    log::error!("{err}");
  }

  Ok(())
}

fn main() {
  setup_logging();

  if let Err(err) = run() {
    log::error!("{err}");
  }
}
