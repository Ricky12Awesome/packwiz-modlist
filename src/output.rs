use log::info;
use tokio::fs::File;
use tokio::io::{stdout, AsyncWrite, AsyncWriteExt};

use crate::args::SortingMode;
use crate::cache::Cache;
use crate::data::{get_data, get_projects};
use crate::object::{Data, Project};
use crate::{Args, GlobalError, GlobalResult, ValidationError};

pub fn display_project(index: usize, format: &str, project: &Project) -> String {
  format
    .replace("{INDEX}", &index.to_string())
    .replace("{TITLE}", &project.title())
    .replace("{NAME}", &project.title())
    .replace("{DESCRIPTION}", &project.description())
    .replace("{SUMMARY}", &project.description())
    .replace("{URL}", &project.url())
    .replace("{ID}", &project.id())
    .replace("{SLUG}", &project.slug())
    .replace("\\n", "\n")
}

pub async fn generate(cache: &mut Cache, args: &Args) -> GlobalResult<Data> {
  let (pack, mods) = get_data(args)?;
  let projects = get_projects(cache, &mods).await?;

  Ok(Data {
    pack,
    mods,
    projects,
  })
}

pub async fn write_projects<W>(args: &Args, data: &Data, writer: &mut W) -> GlobalResult<()>
where
  W: AsyncWrite + Unpin,
{
  let mut projects = data.projects.clone();

  if let Some(mode) = args.sort_by {
    projects.sort_by(|a, b| match mode {
      SortingMode::Name | SortingMode::Title => {
        a.title().to_lowercase().cmp(&b.title().to_lowercase())
      }
      SortingMode::Slug => a.slug().cmp(&b.slug()),
      SortingMode::Id => a.id().cmp(&b.id()),
    });
  }

  if args.reverse {
    projects.reverse();
  }

  for (index, project) in projects.iter().enumerate() {
    let display = display_project(index, &args.format, project);

    info!("{display}");

    writer.write_all(&display.into_bytes()).await?;
  }

  Ok(())
}

pub async fn write(args: &Args, data: &Data) -> GlobalResult<()> {
  match &args.output {
    Some(path) => {
      let path = if args.output_custom {
        path.clone()
      } else {
        args.path.join(&path)
      };

      if path.exists() && !args.force {
        return Err(GlobalError::Validation(
          ValidationError::OutputAlreadyExits(path),
        ));
      }

      let mut file = File::create(path).await?;

      write_projects(args, data, &mut file).await?;
    }
    None => {
      write_projects(args, data, &mut stdout()).await?;
    }
  }

  Ok(())
}
