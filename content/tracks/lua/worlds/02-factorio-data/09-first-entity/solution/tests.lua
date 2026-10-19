local t = require("oxide.test")
local mock = require("mock_factorio")

local function load() return mock.load_mod("mod", { validate = false }) end

t.test("entity_is_defined", function()
  local box = load().container["kit-crate"]
  t.truthy(box, "data:extend a container named kit-crate")
  t.eq(box.inventory_size, 48)
  t.contains(box.flags or {}, "placeable-neutral")
  mock.validate_prototype(box)   -- names the missing field, e.g. the icon
end)

t.test("item_places_the_entity", function()
  local raw = load()
  t.eq(raw.item["kit-crate"].place_result, "kit-crate")
  t.truthy(raw.container["kit-crate"], "the entity must exist")
  mock.validate_references()
end)

t.test("minable_returns_the_item", function()
  local box = load().container["kit-crate"]
  t.is_type(box.minable, "table", "add `minable`")
  t.eq(box.minable.mining_time, 0.2)
  t.eq(box.minable.result, "kit-crate")
  mock.validate_references()
end)

t.test("box_is_symmetric", function()
  local box = load().container["kit-crate"]
  local c, s = box.collision_box, box.selection_box
  t.is_type(c, "table", "add a collision_box")
  t.is_type(s, "table", "add a selection_box")
  t.near(c[1][1], -c[2][1], 1e-9, "collision_box left must mirror right")
  t.near(c[1][2], -c[2][2], 1e-9, "collision_box top must mirror bottom")
  t.truthy(c[2][1] > 0 and c[2][2] > 0, "collision_box must not be empty")
  t.truthy(s[2][1] >= c[2][1] and s[2][2] >= c[2][2], "selection_box must cover the collision_box")
end)
