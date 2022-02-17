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
}
