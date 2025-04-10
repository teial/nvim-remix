use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not a Solidity file")]
    NotSolidity,
    #[error("Cannot create artifacts location: {0}")]
    ArtificatsNotCreated(#[from] io::Error),
    #[error("Compilation failed: {0}")]
    CompilationFailed(String),
}
