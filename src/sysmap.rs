use super::source::{Resolver, SourceError};

use glob::glob;
use std::{fs::read_to_string, path::Path, path::PathBuf};

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

struct Symbol {
  pub address: u64,
  pub name: String,
  pub c: char,
}

impl Symbol {
  pub fn from(line: &str) -> Option<Self> {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() < 3 {
      None
    } else {
      let address: u64 = if let Ok(address) = u64::from_str_radix(parts[0], 16) {
        address
      } else {
        return None;
      };
      let c = parts[1].chars().nth(0).unwrap();
      let name: String = parts[2].into();

      Some(Self { address, name, c })
    }
  }
}
