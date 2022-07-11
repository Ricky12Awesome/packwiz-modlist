use crate::parser::{CurseforgeId, ModrinthId, Parser};
use crate::Error;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PackwizModUpdate {
  pub modrinth: Option<PackwizModUpdateModrinth>,
  pub curseforge: Option<PackwizModUpdateCurseforge>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PackwizModUpdateModrinth {
  pub mod_id: String,
  pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PackwizModUpdateCurseforge {
  pub file_id: u32,
  pub project_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PackwizMod {
  pub name: String,
  pub filename: String,
  pub update: PackwizModUpdate,
}

pub struct PackwizParser {
  pub modrinth_mods: Vec<ModrinthId>,
  pub curseforge_mods: Vec<CurseforgeId>,
}

impl PackwizParser {
  pub fn load_from<T>(directory: T) -> Result<Self, Error>
  where
    T: Into<PathBuf>,
  {
    let parsed_mods = directory
      .into()
      .read_dir()
      .map_err(crate::error!())?
      .map(|entry| entry.map_err(crate::error!()))
      .filter_ok(|entry| entry.file_name().to_string_lossy().ends_with(".pw.toml"))
      .map_ok(|entry| {
        OpenOptions::new()
          .read(true)
          .open(entry.path())
          .map_err(crate::error!())
      })
      .flatten()
      .map_ok(|file| {
        file
          .bytes()
          .collect::<Result<Vec<u8>, _>>()
          .map_err(crate::error!())
      })
      .flatten()
      .map_ok(|bytes| toml::from_slice::<PackwizMod>(&bytes).map_err(crate::error!()))
      .flatten()
      .collect::<Result<Vec<_>, _>>()?;

    let modrinth_mods = parsed_mods
      .clone()
      .into_iter()
      .filter_map(|m| m.update.modrinth)
      .map(|data| ModrinthId {
        version_id: data.version,
        id: data.mod_id,
      })
      .collect();

    let curseforge_mods = parsed_mods
      .into_iter()
      .filter_map(|m| m.update.curseforge)
      .map(|data| CurseforgeId {
        version_id: data.file_id.to_string(),
        id: data.project_id,
      })
      .collect();

    Ok(Self {
      modrinth_mods,
      curseforge_mods,
    })
  }
}

impl Parser for PackwizParser {
  fn get_modrinth_mods(&self) -> Vec<ModrinthId> {
    self.modrinth_mods.clone()
  }

  fn get_curseforge_mods(&self) -> Vec<CurseforgeId> {
    self.curseforge_mods.clone()
  }
}