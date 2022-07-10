use minreq::Request;
use serde::Serialize;

use crate::consts::{CURSEFORGE_API, CURSEFORGE_API_KEY};
use crate::error::Error;
use crate::request::curseforge::items::{Mod, Mods};
use crate::request::post;

pub mod items;

pub fn post_curseforge(endpoint: &str) -> Request {
  post(format!("{CURSEFORGE_API}{endpoint}"))
    .with_header("x-api-key", CURSEFORGE_API_KEY)
}

pub fn get_curseforge_mods<T>(ids: &T) -> Result<Vec<Mod>, Error>
where
  T: IntoIterator<Item = u32> + Serialize,
{
  let response = post_curseforge("/mods")
    .with_json(&serde_json::json!({ "modIds": ids }))
    .map_err(crate::error!())?
    .send()
    .map_err(crate::error!())?;

  crate::request_returns!(response, Mods).map(|mods| mods.data)
}
