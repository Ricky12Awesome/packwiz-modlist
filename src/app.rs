use crate::cache::CacheId;
use crate::parser::{ParsedCurseForgeId, ParsedModrinthId};
use crate::request::{CurseForgeId, ModrinthId};
use crate::{get_modrinth_projects, Cache, Error, Mod, Parser, get_curseforge_mods};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

pub struct App {
  cache: RefCell<Cache>,
  modrinth_mods: Vec<ParsedModrinthId>,
  curseforge_mods: Vec<ParsedCurseForgeId>,
}

impl App {
  pub fn new<P>(cache: Cache, parser: P) -> Self
  where
    P: Parser,
  {
    let (modrinth_mods, curseforge_mods) = parser.get_mods_owned();

    Self {
      cache: RefCell::new(cache),
      modrinth_mods,
      curseforge_mods,
    }
  }

  fn get_mods(&self) -> Result<Vec<Mod>, Error> {
    let mut cache = self.cache.borrow_mut();
    let mut mods = Vec::<Mod>::with_capacity(self.modrinth_mods.len() + self.curseforge_mods.len());
    let mut mr_mods_ids = Vec::<ModrinthId>::with_capacity(self.modrinth_mods.len());
    let mut cf_mods_ids = Vec::<CurseForgeId>::with_capacity(self.curseforge_mods.len());
    let mut mr_id_map = HashMap::<String, CacheId>::with_capacity(mr_mods_ids.capacity());
    let mut cf_id_map = HashMap::<String, CacheId>::with_capacity(cf_mods_ids.capacity());

    for id in self.modrinth_mods.iter().cloned() {
      match cache.get_mod(id.clone()).cloned() {
        None => {
          mr_id_map.insert(id.id.clone(), id.clone().into());
          mr_mods_ids.push(id.into());
        }
        Some(m) => mods.push(m),
      }
    }

    for id in self.curseforge_mods.iter().cloned() {
      match cache.get_mod(id.clone()).cloned() {
        None => {
          cf_id_map.insert(id.id.to_string(), id.clone().into());
          cf_mods_ids.push(id.into());
        }
        Some(m) => mods.push(m),
      }
    }

    if !mr_mods_ids.is_empty() {
      let mr_mods = get_modrinth_projects(mr_mods_ids)?
        .into_iter()
        .map(Mod::from);

      for m in mr_mods {
        let id = mr_id_map.get(&m.id).cloned().unwrap();

        cache.set_mod(id, m.clone());
        mods.push(m);
      }
    }

    if !cf_mods_ids.is_empty() {
      let cf_mods = get_curseforge_mods(cf_mods_ids)?
        .into_iter()
        .map(Mod::from);

      for m in cf_mods {
        let id = cf_id_map.get(&m.id).cloned().unwrap();

        cache.set_mod(id, m.clone());
        mods.push(m);
      }
    }

    Ok(mods)
  }

  pub fn run(&self) -> Result<(), Error> {
    let mods = self.get_mods()?;

    println!("{mods:#?}");
    Ok(())
  }

  pub fn close(&self) -> Result<(), Error> {
    self.cache.borrow().save()
  }
}
