-- Press Run: the expression engine on a few recipe-style formulas.
local calc = require("calc")

local env = {iron_plate = 12, gear_cost = 2}
local demos = {
  "2 + 3 * 4",
  "(iron_plate - 2) / gear_cost",
  "-(2 ^ 3) + 1",
  "10 / (5 - 5)",
  "3 +",
}

print("Recipe calculator: expression engine")
for _, src in ipairs(demos) do
  local ok, result = pcall(calc.eval, src, env)
  if ok then
    print(src .. "  =  " .. tostring(result))
  else
    print(src .. "  ->  " .. tostring(result))
  end
end
