extern crate core;

use std::fmt::Display;

use cache::Cache;
use clap::Parser;
use colored::Colorize;
use simple_logger::SimpleLogger;

use crate::args::{Args, ColorMode};
use crate::error::{handle_error, GlobalError, GlobalResult, ValidationError};
use crate::output::{generate, write};

mod args;
mod cache;
mod data;
mod error;
mod object;
mod output;

#[tokio::main]
async fn main() {
  let args = Args::parse();

  match args.color_mode {
    ColorMode::Auto => (),
    ColorMode::Always => colored::control::set_override(true),
    ColorMode::Never => colored::control::set_override(false),
  }

  #[cfg(windows)]
  colored::control::set_virtual_terminal(true).unwrap();

  SimpleLogger::new()
    .with_level(args.log_level)
    .init()
    .unwrap();

  if args.about {
    fn about(k: &str, v: impl Display) {
      println!("{}{}{}", k.bright_purple(), ": ".white(), v);
    }

    about("Name", env!("CARGO_PKG_NAME").bright_yellow());
    about("Version", env!("CARGO_PKG_VERSION").bright_red());
    about("Author", env!("CARGO_PKG_AUTHORS").bright_yellow());
    about("Description", env!("CARGO_PKG_DESCRIPTION").bright_yellow());
    about("License", env!("CARGO_PKG_LICENSE").bright_cyan());
    about("Repository", env!("CARGO_PKG_REPOSITORY").bright_blue());

    return;
  }

  let result = run(&args).await;

  if let Err(err) = result {
    handle_error(&err);
  }
}

async fn run(args: &Args) -> GlobalResult<()> {
  let mut cache = Cache::new(args).await?;
  let data = generate(&mut cache, args).await?;

  if args.json {
    println!("{}", serde_json::to_string_pretty(&data).unwrap());

    return Ok(());
  }

  write(args, &data).await?;
  cache.save(args).await?;

  Ok(())
}
