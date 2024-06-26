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

impl PackMod {
  pub fn id(&self) -> String {
    if let Some(pack_mod) = &self.update.modrinth {
      pack_mod.mod_id.clone()
    } else if let Some(pack_mod) = &self.update.curseforge {
      pack_mod.project_id.to_string()
    } else {
      unreachable!()
    }
  }

  pub fn hash(&self) -> &String {
    &self.download.hash
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackModDownload {
  pub hash: String,
  #[serde(alias = "hash-format")]
  pub hash_format: String,
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
  #[serde(default)]
  pub authors: Vec<CurseForgeAuthor>,
  pub logo: Option<CurseForgeLogo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct CurseForgeAuthor {
  pub id: u32,
  pub name: String,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseForgeLogo {
  pub id: u32,
  pub mod_id: u32,
  pub title: String,
  pub thumbnail_url: String,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct ModrinthProject {
  pub id: String,
  pub slug: String,
  pub team: String,
  #[serde(default)]
  pub team_members: Vec<ModrinthTeamMember>,
  pub icon_url: Option<String>,
  pub source_url: Option<String>,
  pub title: String,
  pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct ModrinthTeamMember {
  pub role: String,
  pub team_id: String,
  pub user: ModrinthTeamMemberUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct ModrinthTeamMemberUser {
  pub id: String,
  pub username: String,
  pub avatar_url: Option<String>,
  pub bio: Option<String>,
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
      Project::CurseForge(CurseForgeProject { slug, .. }) => {
        format!("https://www.curseforge.com/minecraft/mc-mods/{slug}")
      }
      Project::Modrinth(ModrinthProject { id, .. }) => format!("https://modrinth.com/mod/{id}"),
    }
  }

  pub fn id(&self) -> String {
    match self {
      Project::CurseForge(CurseForgeProject { id, .. }) => id.to_string(),
      Project::Modrinth(ModrinthProject { id, .. }) => id.clone(),
    }
  }

  pub fn slug(&self) -> String {
    match self {
      Project::CurseForge(CurseForgeProject { slug, .. }) => slug.clone(),
      Project::Modrinth(ModrinthProject { slug, .. }) => slug.clone(),
    }
  }

  pub fn title(&self) -> String {
    match self {
      Project::CurseForge(CurseForgeProject { name, .. }) => name.clone(),
      Project::Modrinth(ModrinthProject { title, .. }) => title.clone(),
    }
  }

  pub fn description(&self) -> String {
    match self {
      Project::CurseForge(CurseForgeProject { summary, .. }) => summary.clone(),
      Project::Modrinth(ModrinthProject { description, .. }) => description.clone(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
  pub pack: Pack,
  pub mods: PackMods,
  pub projects: Projects,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeModIds {
  pub mod_ids: Vec<u32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurseforgeMods {
  pub data: Vec<CurseForgeProject>
}