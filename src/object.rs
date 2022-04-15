use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pack {
  pub name: String,
  pub author: String,
  pub version: String,
  #[serde(alias = "pack-format")]
  pub pack_format: String,
  pub versions: PackVersions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackVersions {
  pub fabric: Option<String>,
  pub forge: Option<String>,
  pub minecraft: String,
}

pub type PackMods = Vec<PackMod>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackMod {
  pub name: String,
  pub filename: String,
  pub side: String,
  pub download: PackModDownload,
  pub update: PackModUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackModDownload {
  pub url: String,
  #[serde(alias = "hash-format")]
  pub hash_format: String,
  pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackModUpdate {
  pub curseforge: Option<PackModUpdateCurseforge>,
  pub modrinth: Option<PackModUpdateModrinth>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackModUpdateCurseforge {
  #[serde(alias = "file-id")]
  pub file_id: u32,
  #[serde(alias = "project-id")]
  pub project_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackModUpdateModrinth {
  #[serde(alias = "mod-id")]
  pub mod_id: String,
  #[serde(alias = "project-id")]
  pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct CurseForgeProject {
  pub id: u32,
  pub slug: String,
  pub name: String,
  pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct ModrinthProject {
  pub id: String,
  pub slug: String,
  pub title: String,
  pub description: String,
}

pub type Projects = Vec<Project>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Project {
  CurseForge(CurseForgeProject),
  Modrinth(ModrinthProject),
}

impl From<CurseForgeProject> for Project {
  fn from(project: CurseForgeProject) -> Self {
    Project::CurseForge(project)
  }
}

impl From<ModrinthProject> for Project {
  fn from(project: ModrinthProject) -> Self {
    Project::Modrinth(project)
  }
}

impl Project {
  pub fn url(&self) -> String {
    match self {
      Project::CurseForge(CurseForgeProject { slug, .. }) => format!("https://www.curseforge.com/minecraft/mc-mods/{}", slug),
      Project::Modrinth(ModrinthProject { slug, .. }) => format!("https://modrinth.com/mod/{}", slug)
    }
  }

  pub fn slug(&self) -> String {
    match self {
      Project::CurseForge(CurseForgeProject { slug, .. }) => slug.clone(),
      Project::Modrinth(ModrinthProject { slug, .. }) => slug.clone()
    }
  }

  pub fn title(&self) -> String {
    match self {
      Project::CurseForge(CurseForgeProject { name, .. }) => name.clone(),
      Project::Modrinth(ModrinthProject { title, .. }) => title.clone()
    }
  }

  pub fn description(&self) -> String {
    match self {
      Project::CurseForge(CurseForgeProject { summary, .. }) => summary.clone(),
      Project::Modrinth(ModrinthProject { description, .. }) => description.clone()
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
  pub pack: Pack,
  pub mods: PackMods,
  pub projects: Projects,
}