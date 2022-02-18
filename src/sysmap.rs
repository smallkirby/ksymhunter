use super::source::{Resolver, SourceError, Symbol};

use std::{fs::read_to_string, path::Path};

pub struct SysmapSource {
  pub sfiles: Vec<String>, // sysmap source filename
}

impl Resolver for SysmapSource {
  fn resolve(&self, symbol: &str) -> Result<Option<u64>, SourceError> {
    for sfile in &self.sfiles {
      let content = read_to_string(sfile)?;
      let lines: Vec<&str> = content.split('\n').collect();

      for line in lines {
        if let Some(entry) = Symbol::from(line) {
          if entry.name == symbol {
            return Ok(Some(entry.address));
          }
        }
      }
    }

    Ok(None)
  }
}

impl SysmapSource {
  pub fn from_array(sfiles: Vec<&str>) -> Vec<Self> {
    fn resolver_mapper(source: &str) -> Option<SysmapSource> {
      SysmapSource::new(source).ok()
    }
    sfiles.into_iter().filter_map(resolver_mapper).collect()
  }

  pub fn new(sfile: &str) -> Result<Self, SourceError> {
    let sfiles = if sfile.contains('*') {
      glob::glob(sfile)
        .unwrap()
        .filter(|cand| cand.is_ok() && cand.as_ref().unwrap().as_path().is_file())
        .map(|cand| {
          let pathbuf = cand.unwrap();
          pathbuf.as_path().to_string_lossy().to_string()
        })
        .collect()
    } else {
      if !Path::new(sfile).is_file() {
        return Err(SourceError::SourceNotFound(sfile.into()));
      }
      vec![sfile.into()]
    };

    Ok(SysmapSource { sfiles })
  }
}
