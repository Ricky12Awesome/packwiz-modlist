use crate::consts::MODRINTH_API;
use crate::error::Error;
use crate::request::get;
use minreq::{Request, URL};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
  pub url: String,
}

pub fn get_modrinth(endpoint: &str) -> Request {
  get(format!("{MODRINTH_API}{endpoint}"))
}

pub fn get_modrinth_projects<T>(projects: &T) -> Result<Projects, Error>
where
  T: Serialize,
{
  let json = serde_json::to_string(projects).map_err(crate::error!())?;
  let response = get_modrinth("/projects")
    .with_param("ids", json)
    .send()
    .unwrap();

  match response.status_code {
    200 => response.json().map_err(crate::error!()),
    _ => Err(crate::error!(response)),
  }
}
