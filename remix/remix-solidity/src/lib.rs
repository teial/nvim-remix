use std::fs;
use std::path::Path;
use std::process::Command;

use error::Error;

pub mod error;

#[allow(clippy::nonminimal_bool)]
pub fn compile(filename: impl AsRef<Path>) -> Result<(), Error> {
    if !filename.as_ref().extension().is_some_and(|ext| ext == "sol") {
        return Err(Error::NotSolidity);
    }

    let output_dir = "artifacts";
    fs::create_dir_all(output_dir)?;
    Command::new("solc")
        .arg("--combined-json")
        .arg("abi,bin")
        .arg("--overwrite")
        .arg("--optimize")
        .arg("--output-dir")
        .arg(output_dir)
        .arg(filename.as_ref())
        .output()
        .map_err(|e| Error::CompilationFailed(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn not_solidity() {
        compile("resources/wrong.txt").unwrap();
    }
}
