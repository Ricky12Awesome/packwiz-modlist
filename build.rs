#![allow(unused)]

extern crate clap;
extern crate log;

use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use clap::CommandFactory;
use clap_complete::{generate_to, Generator, Shell};

include!("src/args.rs");

fn generate_completions<G, P>(shell: G, dir: P) -> anyhow::Result<()>
where
  G: Generator,
  P: Into<OsString>,
{
  let mut command = Args::command();

  // uncomment to generate completions,
  // commented because it slows down builds for testing
  generate_to(shell, &mut command, "packwizml", dir)?;

  Ok(())
}

fn main() -> anyhow::Result<()> {
  // generate_completions(Shell::PowerShell, "completions")?;
  // generate_completions(Shell::Bash, "completions")?;
  // generate_completions(Shell::Zsh, "completions")?;
  // generate_completions(Shell::Fish, "completions")?;

  if let Ok(mut file) = File::open(".cf_token") {
    let mut str = String::new();
    file.read_to_string(&mut str);
    println!("cargo:rustc-env=CF_API_KEY={str}");
  }

  Ok(())
}
