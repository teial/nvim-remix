#![allow(unused)]

use nvim_oxi as nvim;

use nvim::Dictionary;
use nvim::api;
use nvim::api::types::LogLevel;

pub fn error(message: &str) -> nvim::Result<()> {
    let opts = Dictionary::default();
    api::notify(message, LogLevel::Error, &opts)?;
    Ok(())
}

pub fn warn(message: &str) -> nvim::Result<()> {
    let opts = Dictionary::default();
    api::notify(message, LogLevel::Warn, &opts)?;
    Ok(())
}

pub fn info(message: &str) -> nvim::Result<()> {
    let opts = Dictionary::default();
    api::notify(message, LogLevel::Info, &opts)?;
    Ok(())
}
