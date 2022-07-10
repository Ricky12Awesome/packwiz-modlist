use std::fmt::Display;

use minreq::{Request, URL};
use serde::Serialize;

use crate::consts::MODRINTH_API;
use crate::error::Error;
use crate::request::get;
use crate::request::modrinth::items::Projects;

pub mod items;

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

  crate::request_returns!(response)
}
