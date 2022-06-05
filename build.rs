extern crate clap;
extern crate log;

use std::ffi::OsString;

use clap::CommandFactory;
use clap_complete::{generate_to, Generator, Shell};

include!("src/args.rs");

fn generate_completions<G, P>(shell: G, dir: P) -> anyhow::Result<()>
where
  G: Generator,
  P: Into<OsString>,
{
  let mut command = Args::command();

  generate_to(shell, &mut command, "packwizml", dir)?;

  Ok(())
}

fn main() -> anyhow::Result<()> {
  generate_completions(Shell::PowerShell, "completions")?;
  generate_completions(Shell::Bash, "completions")?;
  generate_completions(Shell::Zsh, "completions")?;
  generate_completions(Shell::Fish, "completions")?;

  Ok(())
}
