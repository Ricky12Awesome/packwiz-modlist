use log::info;
use tokio::fs::File;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::{Args, GlobalError, GlobalResult, ValidationError};
use crate::data::{get_data, get_projects};
use crate::object::{Data, Project};

pub fn display_project(format: &str, project: &Project) -> String {
  format
    .replace("{TITLE}", &project.title())
    .replace("{NAME}", &project.title())
    .replace("{DESCRIPTION}", &project.description())
    .replace("{SUMMARY}", &project.description())
    .replace("{URL}", &project.url())
    .replace("{SLUG}", &project.slug())
}

pub async fn generate(args: &Args) -> GlobalResult<Data> {
  let (pack, mods) = get_data(args)?;
  let projects = get_projects(&mods).await?;

  Ok(Data { pack, mods, projects })
}

pub async fn write_projects<W>(args: &Args, data: &Data, writer: &mut W) -> GlobalResult<()>
  where W: AsyncWrite + Unpin
{
  for project in &data.projects {
    let display = display_project(&args.format, project);

    info!("{}", &display);

    let display = format!("{}\n", display);

    writer.write_all(&display.into_bytes()).await?;
  }

  Ok(())
}

pub async fn write_to_file(args: &Args, data: &Data) -> GlobalResult<()> {
  let path = if args.output_custom {
    args.output.clone()
  } else {
    args.path.join(&args.output)
  };

  if path.exists() && !args.force {
    return Err(GlobalError::Validation(ValidationError::OutputAlreadyExits(path)));
  }

  let mut file = File::create(path).await?;

  write_projects(args, data, &mut file).await?;

  Ok(())
}