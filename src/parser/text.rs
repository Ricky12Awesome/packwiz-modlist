use crate::parser::{ParsedCurseForgeId, ParsedModrinthId, Parser};
use crate::Error;

#[derive(Debug, Clone)]
pub struct TextParser {
  pub modrinth_mods: Vec<ParsedModrinthId>,
  pub curseforge_mods: Vec<ParsedCurseForgeId>,
}

enum ParsedLine {
  Modrinth(String, String),
  CurseForge(i32, String),
}

#[derive(thiserror::Error, Debug)]
#[error("{}")]
pub enum ParseError {
  #[error("Failed to parse \"{0}\" at line {1}")]
  ParseLine(String, usize),
  #[error("Failed to parse \"{0}\" at line {1}, \"{2}\" is not a valid number")]
  InvalidNumber(String, usize, String),
  #[error("Failed to parse \"{0}\" at line {1}, \"{2}\" is not a valid source")]
  InvalidSource(String, usize, String),
}

fn parse_line((n, line): (usize, &str)) -> Result<ParsedLine, ParseError> {
  let (src, right) = line
    .split_once(':')
    .ok_or_else(|| ParseError::ParseLine(line.into(), n))?;

  let (id, version) = right.split_once(':').unwrap_or((right, ""));

  match src.to_lowercase().as_str() {
    "modrinth" | "mr" => Ok(ParsedLine::Modrinth(id.into(), version.into())),
    "curseforge" | "cf" => Ok(ParsedLine::CurseForge(
      id.parse()
        .map_err(|_| ParseError::InvalidNumber(line.into(), n, id.into()))?,
      version.into(),
    )),
    src => Err(ParseError::InvalidSource(line.into(), n, src.into())),
  }
}

impl TextParser {
  pub fn new<S>(text: S) -> Result<Self, Error>
  where
    S: ToString,
  {
    let result = text
      .to_string()
      .lines()
      .map(|line| line.trim())
      .enumerate()
      .filter(|(_, line)| !line.is_empty())
      .filter(|(_, line)| !line.starts_with('#'))
      .filter(|(_, line)| !line.starts_with("//"))
      .map(parse_line)
      .collect::<Result<Vec<_>, _>>()?;

    let mut modrinth_mods: Vec<ParsedModrinthId> = Vec::with_capacity(result.len());
    let mut curseforge_mods: Vec<ParsedCurseForgeId> = Vec::with_capacity(result.len());

    for line in result {
      match line {
        ParsedLine::Modrinth(id, version_id) => modrinth_mods.push(ParsedModrinthId {
          id,
          cache_id: version_id,
        }),
        ParsedLine::CurseForge(id, version_id) => curseforge_mods.push(ParsedCurseForgeId {
          id,
          cache_id: version_id,
        }),
      }
    }

    Ok(Self {
      modrinth_mods,
      curseforge_mods,
    })
  }
}

impl Parser for TextParser {
  fn get_mods_owned(self) -> (Vec<ParsedModrinthId>, Vec<ParsedCurseForgeId>) {
    (self.modrinth_mods, self.curseforge_mods)
  }

  fn get_modrinth_mods(&self) -> Vec<ParsedModrinthId> {
    self.modrinth_mods.clone()
  }

  fn get_curseforge_mods(&self) -> Vec<ParsedCurseForgeId> {
    self.curseforge_mods.clone()
  }
}
