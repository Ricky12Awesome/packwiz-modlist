use log::info;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::{Args, GlobalError, GlobalResult, ValidationError};
use crate::data::{get_data, get_projects};
use crate::object::Project;

pub fn display_project(format: &str, project: &Project) -> String {
  format
    .replace("{TITLE}", &project.title())
    .replace("{NAME}", &project.title())
    .replace("{DESCRIPTION}", &project.description())
    .replace("{SUMMARY}", &project.description())
    .replace("{URL}", &project.url())
    .replace("{SLUG}", &project.slug())
}

pub async fn generate(args: &Args) -> GlobalResult<()> {
  let path = if args.output_custom {
    args.output.clone()
  } else {
    args.path.join(&args.output)
  };

  if path.exists() && !args.force {
    return Err(GlobalError::Validation(ValidationError::OutputAlreadyExits(path)));
  }

  let format = args.format.clone();
  let (_pack, mods) = get_data(args)?;
  let projects = get_projects(mods).await?;
  let mut file = File::create(path).await?;

  for project in projects {
    let display = display_project(&format, &project);

    info!("{}", &display);

    let display = format!("{}\n", display);

    file.write_all(&display.into_bytes()).await?;
  }

  Ok(())
}