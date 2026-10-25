local t = require("oxide.test")
local mock = require("mock_factorio")

local function load() return mock.load_mod("mod", { validate = false }) end

t.test("item_is_in_the_intermediate_row", function()
  local raw = load()
  local item = raw.item["kit-alloy"]
  t.truthy(item, "kit-alloy is not registered")
  t.eq(item.subgroup, "intermediate-product")
  t.truthy(raw["item-subgroup"][item.subgroup], "subgroup must name an existing item-subgroup")
end)

t.test("item_sorts_after_vanilla", function()
  local raw = load()
  local item = raw.item["kit-alloy"]
  t.is_type(item.order, "string", "set an order string")
  for name, other in pairs(mock.pristine().item) do
    if other.subgroup == "intermediate-product" then
      t.truthy(item.order > other.order, ("%q must sort after %s's %q"):format(item.order, name, other.order))
    end
  end
end)
