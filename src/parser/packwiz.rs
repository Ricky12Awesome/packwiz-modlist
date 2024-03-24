use crate::parser::{ParsedCurseForgeId, ParsedModrinthId, Parser};
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
  pub project_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PackwizMod {
  pub name: String,
  pub filename: String,
  pub update: PackwizModUpdate,
}

#[derive(Debug, Clone)]
pub struct PackwizParser {
  pub modrinth_mods: Vec<ParsedModrinthId>,
  pub curseforge_mods: Vec<ParsedCurseForgeId>,
}

impl PackwizParser {
  pub fn load_from<T>(directory: T) -> Result<Self, Error>
  where
    T: Into<PathBuf>,
  {
    let directory = directory.into();
    let parsed_mods = directory
      .read_dir()?
      .map_ok(|entry| entry)
      .filter_ok(|entry| entry.file_name().to_string_lossy().ends_with(".pw.toml"))
      .map_ok(|entry| std::fs::read_to_string(entry.path()))
      .flatten()
      .map_ok(|data| toml::from_str::<PackwizMod>(&data))
      .flatten()
      .collect::<Result<Vec<_>, _>>()?;

    let modrinth_mods = parsed_mods
      .clone()
      .into_iter()
      .filter_map(|m| m.update.modrinth)
      .map(|data| ParsedModrinthId {
        cache_id: data.version,
        id: data.mod_id,
      })
      .collect();

    let curseforge_mods = parsed_mods
      .into_iter()
      .filter_map(|m| m.update.curseforge)
      .map(|data| ParsedCurseForgeId {
        cache_id: data.file_id.to_string(),
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
  fn get_mods_owned(self) -> (Vec<ParsedModrinthId>, Vec<ParsedCurseForgeId>) {
    (self.modrinth_mods, self.curseforge_mods)
  }

  fn get_modrinth_mods(&self) -> Vec<ParsedModrinthId> {
    self.modrinth_mods.clone()
  }

  fn get_curseforge_mods(&self) -> Vec<ParsedCurseForgeId> {
    self.curseforge_mods.clone()
  }
}
