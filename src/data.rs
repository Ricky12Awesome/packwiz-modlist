use std::fs::{DirEntry, File};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use GlobalError::Validation;
use ValidationError::{DirNotExist, MustBeDir, PackNotFound};

use crate::Args;
use crate::error::{GlobalError, GlobalResult};
use crate::util::read_toml_file;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack {
  pub name: String,
  pub author: String,
  pub version: String,
  #[serde(alias = "pack-format")]
  pub pack_format: String,
  pub versions: PackVersions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackVersions {
  pub fabric: Option<String>,
  pub forge: Option<String>,
  pub minecraft: String,
}

pub type PackMods = Vec<PackMod>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackMod {
  pub name: String,
  pub filename: String,
  pub side: String,
  pub download: PackModDownload,
  pub update: PackModUpdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackModDownload {
  pub url: String,
  #[serde(alias = "hash-format")]
  pub hash_format: String,
  pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackModUpdate {
  pub curseforge: Option<PackModUpdateCurseforge>,
  pub modrinth: Option<PackModUpdateModrinth>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackModUpdateCurseforge {
  #[serde(alias = "file-id")]
  pub file_id: u32,
  #[serde(alias = "project-id")]
  pub project_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackModUpdateModrinth {
  #[serde(alias = "mod-id")]
  pub mod_id: String,
  #[serde(alias = "project-id")]
  pub version: String,
}

#[derive(Debug, Error)]
pub enum ValidationError {
  #[error("{0} does not exist")]
  DirNotExist(PathBuf),
  #[error("{0} must be a directory")]
  MustBeDir(PathBuf),
  #[error("pack.toml was not found in {0}")]
  PackNotFound(PathBuf),
}

pub fn get_mods(args: &Args) -> GlobalResult<PackMods> {
  let path = if args.mods_custom {
    args.mods.clone()
  } else {
    args.path.join(&args.mods)
  };

  match () {
    _ if !path.exists() => Err(Validation(DirNotExist(path))),
    _ if !path.is_dir() => Err(Validation(MustBeDir(path))),
    _ => {
      path.read_dir()?
        .filter_map(|it| it.ok())
        .filter(|it| it.file_name().to_string_lossy().ends_with(".toml"))
        .map(|it| read_toml_file(it.path()))
        .collect::<GlobalResult<PackMods>>()
    }
  }
}

pub fn get_data(args: &Args) -> GlobalResult<(PackMods, Pack)> {
  let path = args.path.clone();
  let pack = path.join("pack.toml");

  match () {
    _ if !path.exists() => Err(Validation(DirNotExist(path))),
    _ if !path.is_dir() => Err(Validation(MustBeDir(path))),
    _ if !pack.is_file() => Err(Validation(PackNotFound(path))),
    _ => {
      let mods = get_mods(args)?;
      let pack = read_toml_file(pack)?;

      Ok((mods, pack))
    }
  }
}