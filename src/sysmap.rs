use super::source::{Resolver, SourceError};

use std::{fs::read_to_string, path::Path};

pub struct SysmapSource {
  pub sfile: String, // sysmap source filename
}

impl Resolver for SysmapSource {
  fn resolve(&self, symbol: &str) -> Result<Option<u64>, SourceError> {
    let content = read_to_string(&self.sfile)?;
    let lines: Vec<&str> = content.split('\n').collect();

    for line in lines {
      if let Some(entry) = Symbol::from(line) {
        if entry.name == symbol {
          return Ok(Some(entry.address));
        }
      }
    }

    Ok(None)
  }
}

impl SysmapSource {
  pub fn new(sfile: &str) -> Result<Self, SourceError> {
    if !Path::new(sfile).is_file() {
      return Err(SourceError::SourceNotFound(sfile.into()));
    }

    Ok(SysmapSource {
      sfile: sfile.into(),
    })
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
