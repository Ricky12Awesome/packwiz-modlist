use crate::error::Error;
use crate::Mod;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::PathBuf;

pub type CacheData = HashMap<String, CacheMod>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheMod {
  pub cache_id: String,
  #[serde(flatten)]
  pub data: Mod,
}

#[derive(Debug, Clone)]
pub struct CacheId {
  cache_id: String,
  mod_id: String,
}

impl From<crate::parser::ParsedModrinthId> for CacheId {
  fn from(id: crate::parser::ParsedModrinthId) -> Self {
    Self {
      cache_id: id.cache_id,
      mod_id: id.id,
    }
  }
}

impl From<crate::parser::ParsedCurseForgeId> for CacheId {
  fn from(id: crate::parser::ParsedCurseForgeId) -> Self {
    Self {
      cache_id: id.cache_id,
      mod_id: id.id.to_string(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Cache {
  file: PathBuf,
  is_dirty: bool,
  data: CacheData,
}

impl Cache {
  pub fn load<T>(file: T) -> Result<Self, Error>
  where
    T: Into<PathBuf>,
  {
    let file = file.into();

    match OpenOptions::new().read(true).open(&file) {
      Ok(reader) => Ok(Self {
        file,
        is_dirty: false,
        data: serde_json::from_reader(reader).map_err(crate::error!())?,
      }),
      Err(err) => match err.kind() {
        ErrorKind::NotFound => Ok(Self {
          file,
          is_dirty: false,
          data: Default::default(),
        }),
        _ => Err(crate::error!(err)),
      },
    }
  }

  pub fn set_data(&mut self, data: CacheData) {
    self.data.extend(data);
    self.is_dirty = true;
  }

  pub fn get_data(&self) -> &CacheData {
    &self.data
  }

  pub fn set_mod<T>(&mut self, id: T, data: Mod)
  where
    T: Into<CacheId>,
  {
    let id = id.into();

    self.is_dirty = true;
    self.data.insert(
      id.mod_id,
      CacheMod {
        cache_id: id.cache_id,
        data,
      },
    );
  }

  pub fn get_mod<T>(&self, id: T) -> Option<&Mod>
  where
    T: Into<CacheId>,
  {
    let id = id.into();
    let m = self.data.get(&id.mod_id)?;

    if id.cache_id == m.cache_id {
      Some(&m.data)
    } else {
      None
    }
  }

  pub fn save(&self) -> Result<(), Error> {
    if self.is_dirty {
      let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&self.file)
        .map_err(crate::error!())?;

      serde_json::to_writer(file, &self.data).map_err(crate::error!())?;
    }

    Ok(())
  }
}
