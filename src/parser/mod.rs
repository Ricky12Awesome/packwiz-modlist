pub mod packwiz;
pub mod text;

#[derive(Debug, Clone)]
pub struct ParsedModrinthId {
  pub cache_id: String,
  pub id: String,
}

#[derive(Debug, Clone)]
pub struct ParsedCurseForgeId {
  pub cache_id: String,
  pub id: i32,
}

pub trait Parser: Sized {
  fn get_mods_owned(self) -> (Vec<ParsedModrinthId>, Vec<ParsedCurseForgeId>);
  fn get_modrinth_mods(&self) -> Vec<ParsedModrinthId>;
  fn get_curseforge_mods(&self) -> Vec<ParsedCurseForgeId>;
}
