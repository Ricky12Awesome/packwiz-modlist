use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use itertools::Itertools;
use reqwest::Client;
use serde::de::DeserializeOwned;

use GlobalError::Validation;

use crate::cache::Cache;
use crate::error::ValidationError::{DirNotExist, MustBeDir, PackNotFound};
use crate::error::{GlobalError, GlobalResult};
use crate::object::{
  CurseForgeProject, CurseforgeModIds, CurseforgeMods, ModrinthProject, Pack, PackMod, PackMods,
  Project,
};
use crate::Args;

const CURSEFORGE_API_KEY: &str = env!("CF_API_KEY");
const CURSEFORGE_API: &str = "https://api.curseforge.com/v1";
const MODRINTH_API: &str = "https://api.modrinth.com/v2";

pub fn read_toml_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> GlobalResult<T> {
  let file = File::open(path)?;
  let bytes = file.bytes().collect::<Result<Vec<_>, _>>()?;

  toml::from_slice::<T>(&bytes).map_err(GlobalError::from)
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
    _ => path
      .read_dir()?
      .filter_map(|it| it.ok())
      .filter(|it| it.file_name().to_string_lossy().ends_with(".toml"))
      .map(|it| read_toml_file(it.path()))
      .collect::<GlobalResult<PackMods>>(),
  }
}

pub fn get_data(args: &Args) -> GlobalResult<(Pack, PackMods)> {
  let path = args.path.clone();
  let pack = path.join("pack.toml");

  match () {
    _ if !path.exists() => Err(Validation(DirNotExist(path))),
    _ if !path.is_dir() => Err(Validation(MustBeDir(path))),
    _ if !pack.is_file() => Err(Validation(PackNotFound(path))),
    _ => {
      let mods = get_mods(args)?;
      let pack = read_toml_file(pack)?;

      Ok((pack, mods))
    }
  }
}

#[allow(unused)]
async fn request_modrinth_project(id: &str) -> GlobalResult<ModrinthProject> {
  let url = format!("{MODRINTH_API}/project/{id}");
  let response = reqwest::get(url).await?;
  let project = response.json().await?;

  Ok(project)
}

async fn request_modrinth_projects(ids: Vec<String>) -> GlobalResult<Vec<ModrinthProject>> {
  let ids = serde_json::to_string(&ids)?;
  let url = format!("{MODRINTH_API}/projects?ids={ids}");
  let response = reqwest::get(url).await?;
  let project = response.json().await?;

  Ok(project)
}

#[allow(unused)]
async fn request_curseforge_project(id: u32) -> GlobalResult<CurseForgeProject> {
  let url = format!("{CURSEFORGE_API}/mods/{id}");
  let response = Client::builder()
    .build()?
    .get(url)
    .header("x-api-key", CURSEFORGE_API_KEY)
    .send()
    .await?;

  let project = response.json().await?;

  Ok(project)
}

async fn request_curseforge_projects(ids: Vec<u32>) -> GlobalResult<Vec<CurseForgeProject>> {
  let ids = CurseforgeModIds { mod_ids: ids };
  let url = format!("{CURSEFORGE_API}/mods");
  let response = Client::builder()
    .build()?
    .post(url)
    .header("x-api-key", CURSEFORGE_API_KEY)
    .json(&ids)
    .send()
    .await?;

  let project: CurseforgeMods = response.json().await?;

  Ok(project.data)
}

#[allow(unused)]
pub async fn request_project(pack_mod: &PackMod) -> GlobalResult<Project> {
  if let Some(pack_mod) = &pack_mod.update.modrinth {
    return request_modrinth_project(&pack_mod.mod_id)
      .await
      .map(Project::from);
  }

  if let Some(pack_mod) = &pack_mod.update.curseforge {
    return request_curseforge_project(pack_mod.project_id)
      .await
      .map(Project::from);
  }

  unreachable!()
}

pub async fn get_modrinth_projects(
  cache: &mut Cache,
  mods: &PackMods,
) -> GlobalResult<Vec<Project>> {
  let mut modrinth = Vec::with_capacity(mods.len());

  let filter = mods.iter().filter(|it| it.update.modrinth.is_some());

  match cache.get_all(filter.clone()) {
    Some(projects) => modrinth.extend(projects.into_iter().cloned()),
    None => {
      let lookup = filter
        .clone()
        .into_group_map_by(|it| it.id())
        .into_iter()
        .map(|(key, value)| (key, value[0]))
        .collect::<HashMap<_, _>>();

      let modrinth_ids = filter.map(|it| it.id()).collect();
      let projects = request_modrinth_projects(modrinth_ids)
        .await?
        .into_iter()
        .map(|it| (lookup[&it.id], Project::from(it)));

      cache.insert_all(projects.clone());
      modrinth.extend(projects.map(|it| it.1));
    }
  };

  Ok(modrinth)
}

pub async fn get_curseforge_projects(
  cache: &mut Cache,
  mods: &PackMods,
) -> GlobalResult<Vec<Project>> {
  let mut curseforge = Vec::with_capacity(mods.len());

  let filter = mods.iter().filter(|it| it.update.curseforge.is_some());

  match cache.get_all(filter.clone()) {
    Some(projects) => curseforge.extend(projects.into_iter().cloned()),
    None => {
      let lookup = filter
        .clone()
        .into_group_map_by(|it| it.id())
        .into_iter()
        .map(|(key, value)| (key, value[0]))
        .collect::<HashMap<_, _>>();

      let curseforge_ids = filter.filter_map(|it| it.id().parse().ok()).collect();
      let projects = request_curseforge_projects(curseforge_ids)
        .await?
        .into_iter()
        .map(|it| (lookup[&it.id.to_string()], Project::from(it)));

      cache.insert_all(projects.clone());
      curseforge.extend(projects.map(|it| it.1));
    }
  };

  Ok(curseforge)
}

pub async fn get_projects(cache: &mut Cache, mods: &PackMods) -> GlobalResult<Vec<Project>> {
  let mut projects = Vec::with_capacity(mods.len());

  let modrinth = get_modrinth_projects(cache, mods).await?;
  let curseforge = get_curseforge_projects(cache, mods).await?;

  projects.extend_from_slice(&modrinth);
  projects.extend_from_slice(&curseforge);

  Ok(projects)
}
