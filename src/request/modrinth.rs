use crate::consts::MODRINTH_API;
use crate::error::Error;
use crate::request::{get, ModrinthId};
use minreq::Request;
use serde::{Deserialize, Serialize};

pub type Projects = Vec<Project>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
  pub id: String,
  pub slug: String,
  pub title: String,
  pub description: String,
  pub body: String,
  pub team: String,
  pub icon_url: Option<String>,
  pub issue_url: Option<String>,
  pub source_url: Option<String>,
  pub wiki_url: Option<String>,
  pub license: ProjectLicense,
  pub versions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectLicense {
  pub id: String,
  pub name: String,
  pub url: Option<String>,
}

pub fn get_modrinth(endpoint: &str) -> Request {
  get(format!("{MODRINTH_API}{endpoint}"))
}

pub fn get_modrinth_projects(projects: Vec<ModrinthId>) -> Result<Projects, Error> {
  let json = serde_json::to_string(&projects)?;
  let response = get_modrinth("/projects")
    .with_param("ids", json)
    .send()
    .unwrap();

  match response.status_code {
    200 => response
      .json()
      .map_err(|err| match response.as_str() {
        Ok(json) => (json, err).into(),
        Err(err) => err.into(),
      }),
    _ => Err(response.into()),
  }
}
