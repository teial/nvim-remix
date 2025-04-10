use nvim_oxi as nvim;

use nvim::api;
use nvim::{Dictionary, Function, Object};

mod notify;

#[nvim::plugin]
fn libremix() -> nvim::Result<Dictionary> {
    let solidity_compile: Function<(), Result<(), nvim::Error>> = Function::from_fn(|_| {
        let buffer = api::get_current_buf();
        let filename = buffer.get_name()?;
        match remix_solidity::compile(&filename) {
            Ok(_) => notify::info("Solidity compilation successful.")?,
            Err(e) => notify::error(&e.to_string())?,
        }
        Ok(())
    });

    let api = Dictionary::from_iter([("solidity_compile", Object::from(solidity_compile))]);
    Ok(api)
}
