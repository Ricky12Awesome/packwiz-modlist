use futures::future::JoinAll;

use GlobalError::Validation;

use crate::Args;
use crate::error::{GlobalError, GlobalResult};
use crate::error::ValidationError::{DirNotExist, MustBeDir, PackNotFound};
use crate::misc::read_toml_file;
use crate::object::{CurseForgeProject, ModrinthProject, Pack, PackMod, PackMods, Project};

const CURSEFORGE_API: &str = "https://addons-ecs.forgesvc.net/api/v2";
const MODRINTH_API: &str = "https://api.modrinth.com/v2";

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
pub async fn get_modrinth_project(id: String) -> GlobalResult<ModrinthProject> {
  let url = format!("{MODRINTH_API}/project/{id}");
  let response = reqwest::get(url).await?;
  let project = response.json().await?;

  Ok(project)
}

pub async fn get_modrinth_projects(ids: Vec<String>) -> GlobalResult<Vec<ModrinthProject>> {
  let ids = serde_json::to_string(&ids)?;
  let url = format!("{MODRINTH_API}/projects?ids={ids}");
  let response = reqwest::get(url).await?;
  let project = response.json().await?;

  Ok(project)
}

pub async fn get_curseforge_project(id: u32) -> GlobalResult<CurseForgeProject> {
  let url = format!("{CURSEFORGE_API}/addon/{id}");
  let response = reqwest::get(url).await?;
  let project = response.json().await?;

  Ok(project)
}

#[allow(unused)]
pub async fn get_project(pack_mod: PackMod) -> GlobalResult<Project> {
  if let Some(pack_mod) = pack_mod.update.modrinth {
    return get_modrinth_project(pack_mod.mod_id).await.map(Project::from);
  }

  if let Some(pack_mod) = pack_mod.update.curseforge {
    return get_curseforge_project(pack_mod.project_id).await.map(Project::from);
  }

  unreachable!()
}

pub async fn get_projects(mods: &PackMods) -> GlobalResult<Vec<Project>> {
  let modrinth_ids = mods.iter()
    .filter_map(|it| it.update.modrinth.clone())
    .map(|it| it.mod_id)
    .collect::<Vec<_>>();

  let modrinth = get_modrinth_projects(modrinth_ids)
    .await?
    .into_iter()
    .map(Project::from)
    .collect::<Vec<_>>();

  // Don't know how to do batch calls with curseforge api since there isn't much documentation on it
  let curseforge = mods.iter()
    .filter_map(|it| it.update.curseforge.clone())
    .map(|it| it.project_id)
    .map(get_curseforge_project)
    .collect::<JoinAll<_>>()
    .await
    .into_iter()
    .map(|it| it.map(Project::from))
    .collect::<GlobalResult<Vec<_>>>()?;

  let mut projects = Vec::with_capacity(modrinth.len() + curseforge.len());

  projects.extend(modrinth);
  projects.extend(curseforge);

  Ok(projects)
}