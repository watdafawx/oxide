local t = require("oxide.test")
local mock = require("mock_factorio")

local function load() return mock.load_mod("mod", { validate = false }) end

t.test("item_is_registered", function()
  local raw = load()
  local item = raw.item["kit-alloy"]
  t.truthy(item, "data:extend an item named kit-alloy")
  t.eq(item.type, "item")
  t.truthy(raw["item-subgroup"][item.subgroup], "subgroup must name an existing item-subgroup")
  t.eq(item.subgroup, "intermediate-product")
  t.is_type(item.order, "string", "set an order string")
end)

t.test("item_has_mandatory_fields", function()
  local item = load().item["kit-alloy"]
  t.truthy(item, "kit-alloy is not registered yet")
  mock.validate_prototype(item)   -- raises and names the missing field
end)

t.test("stack_size_is_set", function()
  local item = load().item["kit-alloy"]
  t.truthy(item, "kit-alloy is not registered yet")
  t.eq(item.stack_size, 100)
end)
