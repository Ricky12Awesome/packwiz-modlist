pub mod packwiz;
pub mod text;

#[derive(Debug, Clone)]
pub struct ModrinthId {
  pub version_id: String,
  pub id: String,
}

#[derive(Debug, Clone)]
pub struct CurseForgeId {
  pub version_id: String,
  pub id: i32,
}

pub trait Parser {
  fn get_modrinth_mods(&self) -> Vec<ModrinthId>;
  fn get_curseforge_mods(&self) -> Vec<CurseForgeId>;
}
