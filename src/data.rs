use GlobalError::Validation;

use crate::Args;
use crate::error::{GlobalError, GlobalResult};
use crate::error::ValidationError::{DirNotExist, MustBeDir, PackNotFound};
use crate::misc::read_toml_file;
use crate::object::{CurseForgeProject, ModrinthProject, Pack, PackMods};

const CURSEFORGE_API: &str = "https://addons-ecs.forgesvc.net/api/v2/addon";
const MODRINTH_API: &str = "https://api.modrinth.com/v2/project";

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

pub async fn get_curseforge_project(id: u32) -> GlobalResult<CurseForgeProject> {
  let url = format!("{CURSEFORGE_API}/{id}");
  let response = reqwest::get(url).await?;
  let project = response.json::<CurseForgeProject>().await?;

  Ok(project)
}

pub async fn get_modrinth_project(id: String) -> GlobalResult<ModrinthProject> {
  let url = format!("{MODRINTH_API}/{id}");
  let response = reqwest::get(url).await?;
  let project = response.json::<ModrinthProject>().await?;

  Ok(project)
}