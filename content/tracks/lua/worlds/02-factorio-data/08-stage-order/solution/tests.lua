local t = require("oxide.test")
local mock = require("mock_factorio")

-- `mod` is yours, `other` is the pretend other mod; yours loads first.
local function load() return mock.load_mods({ "mod", "other" }) end

local function position(log, entry)
  for i, e in ipairs(log) do
    if e == entry then return i end
  end
end

t.test("updates_run_after_data", function()
  local raw = load()
  t.truthy(position(mock.stage_log, "other:data") < position(mock.stage_log, "mod:data-updates"),
    "every data.lua runs before any data-updates.lua")
  t.eq(raw.item["other-widget"].subgroup, "kit-parts", "move other-widget into kit-parts in data-updates.lua")
  t.truthy(raw["item-subgroup"]["kit-parts"], "kit-parts must exist")
end)

t.test("final_fixes_sees_other_prototypes", function()
  local raw = load()
  t.eq(raw.item["other-widget"].stack_size, 50, "other-widget starts at 100 and must end at 50")
end)

t.test("change_is_applied_once", function()
  local raw = load()
  local vanilla = mock.pristine()
  for name, item in pairs(vanilla.item) do
    local expected = item.stack_size > 50 and item.stack_size / 2 or item.stack_size
    t.eq(raw.item[name].stack_size, expected, name .. " stack size")
  end
  local runs = 0
  for _, entry in ipairs(mock.stage_log) do
    if entry == "mod:data-final-fixes" then runs = runs + 1 end
  end
  t.eq(runs, 1, "your data-final-fixes.lua runs once per load")
end)
