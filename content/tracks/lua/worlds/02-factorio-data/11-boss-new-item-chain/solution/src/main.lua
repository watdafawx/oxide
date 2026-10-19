-- Press Run: loads mod/data.lua through the mock and prints what you built.
local mock = require("mock_factorio")
local raw = mock.load_mod("mod")

-- The recipe calculator from the start of this track, now fed by your own prototypes.
local function recipe_for(item)
  for _, r in pairs(raw.recipe) do
    for _, res in ipairs(r.results or {}) do
      if res.name == item then return r, res.amount end
    end
  end
end

local function raw_cost(item, amount, totals)
  local r, per_craft = recipe_for(item)
  if not r then
    totals[item] = (totals[item] or 0) + amount
    return
  end
  for _, ing in ipairs(r.ingredients or {}) do
    raw_cost(ing.name, ing.amount * amount / per_craft, totals)
  end
end

local function names(bucket)
  local list = {}
  for name in pairs(bucket or {}) do
    if name:find("^kit%-") then list[#list + 1] = name end
  end
  table.sort(list)
  return list
end

print("Kit Mod prototypes")
for _, kind in ipairs({ "item", "recipe", "container", "technology" }) do
  for _, name in ipairs(names(raw[kind])) do
    print(("  %-10s %s"):format(kind, name))
  end
end

if raw.item["kit-crate"] and recipe_for("kit-crate") then
  local totals = {}
  raw_cost("kit-crate", 1, totals)
  local parts = {}
  for name, n in pairs(totals) do parts[#parts + 1] = ("%g %s"):format(n, name) end
  table.sort(parts)
  print("One kit-crate costs: " .. table.concat(parts, ", "))
end
