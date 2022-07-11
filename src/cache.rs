use crate::error::Error;
use crate::Mod;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::PathBuf;

pub type CacheData = HashMap<String, Mod>;

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
          data: HashMap::new(),
        }),
        _ => Err(crate::error!(err)),
      },
    }
  }

  pub fn write_all(&mut self, data: CacheData) {
    self.data = data;
    self.is_dirty = true;
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
