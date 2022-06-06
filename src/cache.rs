use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::GlobalError::Validation;
use crate::ValidationError::MustBeFile;
use crate::{
  args::Args,
  error::GlobalResult,
  object::{PackMod, Project},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cache(HashMap<String, CacheProject>);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheProject {
  hash: String,
  project: Project,
}

impl Cache {
  pub async fn new(args: &Args) -> GlobalResult<Self> {
    let path = args.cache.clone();

    let mut file = match () {
      _ if path.is_dir() => return Err(Validation(MustBeFile(path))),
      _ if !path.exists() => {
        let mut file = OpenOptions::new()
          .write(true)
          .create(true)
          .open(&args.cache)
          .await?;

        file.write_all(b"{}").await?;
        file.flush().await?;
        file.sync_all().await?;

        OpenOptions::new().read(true).open(&args.cache).await?
      }
      _ => OpenOptions::new().read(true).open(&args.cache).await?,
    };

    let size = file.metadata().await?.len() as _;
    let mut bytes = Vec::with_capacity(size);

    file.read_to_end(&mut bytes).await?;

    let hashmap = serde_json::from_slice(&bytes)?;

    Ok(Self(hashmap))
  }

  pub async fn save(&self, args: &Args) -> GlobalResult<()> {
    let mut file = OpenOptions::new().write(true).open(&args.cache).await?;
    let bytes = serde_json::to_vec_pretty(self)?;

    file.write_all(&bytes).await?;
    file.flush().await?;

    Ok(())
  }

  pub fn insert(&mut self, pack_mod: &PackMod, project: Project) {
    let id = pack_mod.id();
    let hash = pack_mod.hash().clone();
    let value = CacheProject { hash, project };

    self.0.insert(id, value);
  }

  pub fn insert_all<'a, I>(&mut self, mods: I)
  where
    I: IntoIterator<Item = (&'a PackMod, Project)>,
  {
    mods.into_iter().for_each(|it| self.insert(it.0, it.1));
  }

  pub fn get_all<'a, I>(&self, mods: I) -> Option<Vec<&Project>>
  where
    I: IntoIterator<Item = &'a PackMod>,
  {
    mods.into_iter().map(|it| self.get(it)).collect()
  }

  pub fn get(&self, pack_mod: &PackMod) -> Option<&Project> {
    let CacheProject { hash, project } = self.0.get(&pack_mod.id())?;

    if hash == pack_mod.hash() {
      Some(project)
    } else {
      None
    }
  }
}
