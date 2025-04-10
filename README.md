# nvim-remix

A Neovim plugin written in Rust for compiling and interacting with Solidity smart contracts.

## Features

- Compile Solidity contracts using `solc`
- Outputs ABI and bytecode to `./artifacts`

## Usage

1. Open a `.sol` file.
2. Run `:Web3Compile`

## Installation

Using Lazy.nvim:

```lua
{
  "yourname/nvim-remix",
  build = "cargo build --release",
  config = function()
    -- any plugin-specific config
  end,
}
```
