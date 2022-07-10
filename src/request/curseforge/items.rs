use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mods {
  pub data: Vec<Mod>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
  pub id: u32,
  pub name: String,
  pub slug: String,
  pub links: ModLinks,
  pub summary: String,
  pub authors: Vec<ModAuthor>,
  pub logo: ModLogo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModLinks {
  pub website_url: String,
  pub wiki_url: String,
  pub issues_url: String,
  pub source_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModAuthor {
  pub id: u32,
  pub name: String,
  pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModLogo {
  pub id: u32,
  pub mod_id: u32,
  pub title: String,
  pub description: String,
  pub thumbnail_url: String,
}
