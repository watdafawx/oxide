local t = require("oxide.test")
local mock = require("mock_factorio")

-- `mod` is the Kit Mod; `kit-extras` depends on it, so it loads after.
local function load() return mock.load_mods({ "mod", "kit-extras" }) end

t.test("addon_item_joins_kit_parts", function()
  local raw = load()
  t.truthy(raw["item-subgroup"]["kit-parts"], "keep the kit-parts subgroup in data.lua")
  t.eq(raw.item["kit-spring"].subgroup, "kit-parts",
    "kit-spring does not exist yet when your data.lua runs: move the loop to data-updates.lua")
end)

t.test("own_items_join_and_vanilla_stays", function()
  local raw = load()
  t.truthy(raw.item["kit-alloy"], "keep the kit-alloy item in data.lua")
  t.eq(raw.item["kit-alloy"].subgroup, "kit-parts")
  for name, item in pairs(mock.pristine().item) do
    t.eq(raw.item[name].subgroup, item.subgroup, name .. " must keep its vanilla row")
  end
end)
