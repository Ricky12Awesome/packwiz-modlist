use crate::consts::USER_AGENT;
use minreq::{Request, URL};
use serde::{Deserialize, Serialize};

pub mod curseforge;
pub mod modrinth;

pub fn get<T: Into<URL>>(url: T) -> Request {
  minreq::get(url)
    .with_header("User-Agent", USER_AGENT)
    .with_header("Content-Type", "application/json")
    .with_header("Accept", "application/json")
}

pub fn post<T: Into<URL>>(url: T) -> Request {
  minreq::post(url)
    .with_header("User-Agent", USER_AGENT)
    .with_header("Content-Type", "application/json")
    .with_header("Accept", "application/json")
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
  pub id: String,
  pub slug: String,
  pub title: String,
  pub description: String,
  /// [Option] because Curseforge doesn't offer a simple way to get a
  /// license for a project even though it's right on the project page
  pub license: Option<License>,
  /// Will be empty since modrinth handles authors in a different way
  /// by using teams, and currently there is no way to bulk get teams
  /// https://github.com/modrinth/labrinth/issues/331
  pub authors: Vec<Author>,
  pub icon_url: Option<String>,
  pub source_url: Option<String>,
  pub issues_url: Option<String>,
  pub wiki_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Author {
  pub name: String,
  pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct License {
  pub id: String,
  pub name: String,
  pub url: String,
}

impl From<modrinth::Project> for Mod {
  fn from(project: modrinth::Project) -> Self {
    Self {
      id: project.id,
      slug: project.slug,
      title: project.title,
      description: project.description,
      license: Some(License {
        id: project.license.id,
        name: project.license.name,
        url: project.license.url,
      }),
      authors: Vec::new(),
      icon_url: project.icon_url,
      source_url: project.source_url,
      issues_url: project.issue_url,
      wiki_url: project.wiki_url,
    }
  }
}

impl From<curseforge::Mod> for Mod {
  fn from(project: curseforge::Mod) -> Self {
    Self {
      id: project.id.to_string(),
      slug: project.slug,
      title: project.name,
      description: project.summary,
      license: None,
      authors: project
        .authors
        .into_iter()
        .map(|author| Author {
          name: author.name,
          url: author.url,
        })
        .collect(),
      icon_url: project.logo.thumbnail_url.into(),
      source_url: project.links.source_url,
      issues_url: project.links.issues_url,
      wiki_url: project.links.wiki_url,
    }
  }
}
