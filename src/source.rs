use thiserror::Error;

use std::io;

pub trait Resolver {
  fn resolve(&self, symbol: &str) -> Result<Option<u64>, SourceError>;
}

#[derive(Error, Debug)]
pub enum SourceError {
  #[error("Source file not found: {0}.")]
  SourceNotFound(String),
  #[error("Failed to read source")]
  SourceNotReadable(#[from] io::Error),
  #[error("External command failed")]
  ExternalCommandError(),
}

pub struct Symbol {
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
      let c = parts[1].chars().next().unwrap();
      let name: String = parts[2].into();

      Some(Self { address, name, c })
    }
  }
}
