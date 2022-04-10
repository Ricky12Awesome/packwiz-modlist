use std::path::PathBuf;
use clap::Parser;
use simple_logger::SimpleLogger;

use crate::data::get_data;
use crate::error::{error_handler, GlobalResult};

mod data;
mod error;
mod util;

// const CURSEFORGE_API: &str = "https://addons-ecs.forgesvc.net/api/v2/addon";
// const MODRINTH_API: &str = "https://api.modrinth.com/v2/project";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  /// Path to the packwiz directory containing "pack.toml"
  #[clap(long, default_value = "./", value_hint = clap::ValueHint::DirPath)]
  path: PathBuf,
  /// Path to the directory contains all the mod metadata files
  #[clap(long, default_value = "mods", value_hint = clap::ValueHint::DirPath)]
  mods: PathBuf,
  /// Should mod path be it's own path instead of being relative to --path
  #[clap(long, short = 'c')]
  mods_custom: bool,
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
  let (mods, pack) = get_data(&args)?;

  println!("{pack:#?}");
  println!("{:#?}", mods.into_iter().map(|it| it.name).collect::<Vec<_>>());

  Ok(())
}

#[tokio::main]
async fn main() {
  SimpleLogger::new().init().unwrap();

  if let Err(err) = _main().await {
    error_handler(err);
  }
}
