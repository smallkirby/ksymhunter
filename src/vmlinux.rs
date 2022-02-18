use crate::source::{Resolver, SourceError, Symbol};

use std::path::Path;
use std::process::Command;

pub struct VmlinuxSource {
  pub vfiles: Vec<String>, // vmlinux source filename
}

impl Resolver for VmlinuxSource {
  fn resolve(&self, symbol: &str) -> Result<Option<u64>, SourceError> {
    for vfile in &self.vfiles {
      let result = Command::new("nm").arg(vfile).output()?;
      if !result.status.success() {
        // just ignore error
        continue;
      }
      let nmoutput = String::from_utf8(result.stdout).unwrap();
      let lines: Vec<&str> = nmoutput.split('\n').collect();
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

impl VmlinuxSource {
  pub fn new(vfile: &str) -> Result<Self, SourceError> {
    let vfiles = if vfile.contains('*') {
      glob::glob(vfile)
        .unwrap()
        .filter(|cand| cand.is_ok() && cand.as_ref().unwrap().as_path().is_file())
        .map(|cand| {
          let pathbuf = cand.unwrap();
          pathbuf.as_path().to_string_lossy().to_string()
        })
        .collect()
    } else {
      if !Path::new(vfile).is_file() {
        return Err(SourceError::SourceNotFound(vfile.into()));
      }
      vec![vfile.into()]
    };

    Ok(VmlinuxSource { vfiles })
  }

  pub fn from_array(sfiles: Vec<&str>) -> Vec<Self> {
    fn resolver_mapper(source: &str) -> Option<VmlinuxSource> {
      VmlinuxSource::new(source).ok()
    }
    sfiles.into_iter().filter_map(resolver_mapper).collect()
  }
}
