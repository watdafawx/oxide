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

t.test("runs_to_the_end", function()
  local lines = run_program()
  t.eq(#lines, 3, "the screen should show three lines")
end)

t.test("shows_the_price_third", function()
  local lines = run_program()
  t.eq(lines[3], "Bikes from 10 euros a day", "the third line of the screen")
end)
