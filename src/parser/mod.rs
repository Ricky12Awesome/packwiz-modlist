pub mod packwiz;

#[derive(Debug, Clone)]
pub struct ModrinthId {
  pub version_id: String,
  pub id: String,
}

#[derive(Debug, Clone)]
pub struct CurseforgeId {
  pub version_id: String,
  pub id: u32,
}

pub trait Parser {
  fn get_modrinth_mods(&self) -> Vec<ModrinthId>;
  fn get_curseforge_mods(&self) -> Vec<CurseforgeId>;
}
