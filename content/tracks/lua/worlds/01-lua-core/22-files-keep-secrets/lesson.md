The calculator is several files now. A recipe registry that hands its inner table to every other file lets any of them break it by accident. A file should show a small front and keep the rest to itself.

**One file, one table.** The last line `return M` is what the file gives to anyone who asks for it. `require("registry")` runs `registry.lua` once, remembers the result and hands back that table.

**Public is on `M`.** Everything you hang on `M`, like `M.register`, can be reached from other files. That is the file's public face.

**Private is `local`.** A `local` written at the top of a file is visible to the functions in that file and to nobody else. Other files cannot see it, read it or replace it.

```lua
local count = 0              -- private: only this file sees it

local M = {}

function M.bump()
  count = count + 1
  return count
end

return M
```

**Hide the data.** The functions on `M` reach `count` from inside, so they work, while the number itself stays out of reach. The registry's table of recipes should live the same way.

## Your turn

- In `registry.lua`, replace `M.recipes = {}` with a `local recipes = {}` above `local M`.
- Fill in `register` so it stores a recipe under its name.
- Fill in `get` so it returns the recipe stored under a name, or `nil`.
