use std::path::PathBuf;

use clap::Parser;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

use crate::data::{get_curseforge_project, get_data, get_modrinth_project};
use crate::error::{error_handler, GlobalResult};

mod data;
mod error;
mod misc;
mod object;
mod output;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  /// Path to the packwiz directory containing "pack.toml"
  #[clap(long, short = 'p', default_value = "./", value_hint = clap::ValueHint::DirPath)]
  path: PathBuf,
  /// Path to the directory contains all the mod metadata files
  #[clap(long, short = 'm', default_value = "mods", value_hint = clap::ValueHint::DirPath)]
  mods: PathBuf,
  /// Should mod path be it's own path instead of being relative to --path
  #[clap(long, short = 'c')]
  mods_custom: bool,
  /// Output file
  #[clap(long, short = 'o', default_value = "modlist.md")]
  output: PathBuf,
  /// Format
  // TODO: Implement Format
  #[clap(long, short = 'f', default_value = "({NAME})[{URL}] - {DESCRIPTION}")]
  format: String,
  /// Override the minecraft version
  #[clap(long)]
  mc: Option<String>,
  /// Override the fabric version
  #[clap(long)]
  fabric: Option<String>,
  /// Override the forge version
  #[clap(long)]
  forge: Option<String>,
}

async fn _main() -> GlobalResult<()> {
  let args = Args::try_parse()?;
  let (mods, _pack) = get_data(&args)?;

  let curseforge_mods = mods.iter()
    .filter_map(|it| it.update.curseforge.clone())
    .map(|it| it.project_id)
    .collect::<Vec<_>>();

  let modrinth_mods = mods.iter()
    .filter_map(|it| it.update.modrinth.clone())
    .map(|it| it.mod_id)
    .collect::<Vec<_>>();

  // println!("{curseforge_mods:?}");
  // println!("{modrinth_mods:?}");

  for curseforge_mod in curseforge_mods {
    let project = get_curseforge_project(curseforge_mod).await?;
    info!("{project:?}");
  }

  for modrinth_mod in modrinth_mods {
    let project = get_modrinth_project(modrinth_mod).await?;
    info!("{project:?}");
  }

  Ok(())
}

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  const LEVEL: LevelFilter = LevelFilter::Debug;
  #[cfg(not(debug_assertions))]
  const LEVEL: LevelFilter = LevelFilter::Info;

  SimpleLogger::new().with_level(LEVEL).init().unwrap();

  if let Err(err) = _main().await {
    error_handler(err);
  }
}
