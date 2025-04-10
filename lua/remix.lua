local plugin_path = debug.getinfo(1, "S").source:sub(2)
local plugin_dir = plugin_path:match("(.*/)")
local shared_lib_dir = plugin_dir .. "../remix/target/release/"

-- Add the build directory to package.cpath
package.cpath = shared_lib_dir .. "?.so;" .. package.cpath -- Linux/macOS
package.cpath = shared_lib_dir .. "?.dylib;" .. package.cpath -- macOS
package.cpath = shared_lib_dir .. "?.dll;" .. package.cpath -- Windows

-- Try loading the compiled plugin
local ok, mod = pcall(require, "libremix")
if not ok then
    vim.notify("Failed to load libremix: " .. mod, vim.log.levels.ERROR, {})
end

return mod
