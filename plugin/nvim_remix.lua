local remix = require("remix")
vim.api.nvim_create_user_command("Web3Compile", function()
    remix.solidity_compile()
end, {})
