-- Press Run: the formula engine on a few recipe-style formulas.
local calc = require("calc")

local env = {eggs = 12, batches = 3}
local demos = {
  "2 + 3 * 4",
  "(eggs - 2) / batches",
  "-(2 ^ 3) + 1",
  "10 / (5 - 5)",
  "3 +",
}

print("Recipe calculator: formula engine")
for _, src in ipairs(demos) do
  local ok, result = pcall(calc.eval, src, env)
  if ok then
    print(src .. "  =  " .. tostring(result))
  else
    print(src .. "  ->  " .. tostring(result))
  end
end
