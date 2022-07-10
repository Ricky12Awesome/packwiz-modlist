use serde::{Deserialize, Serialize};

pub type Projects = Vec<Project>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
  id: String,
  slug: String,
  title: String,
  description: String,
  body: String,
  team: String,
  license: ProjectLicense,
  versions: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectLicense {
  id: String,
  name: String,
  url: String,
}