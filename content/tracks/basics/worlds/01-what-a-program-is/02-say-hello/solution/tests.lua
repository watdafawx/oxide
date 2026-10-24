local t = require("oxide.test")

-- Runs src/main.lua and collects every line it prints.
local function run_program()
  local lines = {}
  local show = print
  print = function(...)
    local parts = {}
    for i = 1, select("#", ...) do parts[#parts + 1] = tostring((select(i, ...))) end
    lines[#lines + 1] = table.concat(parts, "\t")
    show(...)
  end
  package.loaded["main"] = nil
  local ok, err = pcall(require, "main")
  print = show
  if not ok then
    local msg = tostring(err):gsub("\n.*", "")
    error(msg:match("(src/main%.lua:.*)") or msg, 0)
  end
  return lines
end

t.test("greets_first", function()
  local lines = run_program()
  t.eq(lines[1], "Welcome to the bike shop!", "the first line of the screen")
end)

t.test("shows_the_hours_second", function()
  local lines = run_program()
  t.eq(lines[2], "Open 9 to 5", "the second line of the screen")
end)
