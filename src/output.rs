use crate::{Args, GlobalError, GlobalResult, ValidationError};
use crate::data::{get_data, get_projects};

pub fn format(format: String, text: String) {

}

pub async fn generate(args: &Args) -> GlobalResult<()> {
  if args.output.exists() {
    return Err(GlobalError::Validation(ValidationError::OutputAlreadyExits(args.output.clone())));
  }

  let (pack, mods) = get_data(args)?;
  let projects = get_projects(mods).await?;

  println!("{projects:#?}");

  Ok(())
}