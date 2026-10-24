local t = require("oxide.test")
local mock = require("mock_factorio")

local function box() return mock.load_mod("mod", { validate = false }).container["kit-crate"] end

t.test("collision_box_is_centred", function()
  local c = box().collision_box
  t.is_type(c, "table", "add a collision_box")
  t.near(c[1][1], -c[2][1], 1e-9, "left must mirror right")
  t.near(c[1][2], -c[2][2], 1e-9, "top must mirror bottom")
  t.truthy(c[2][1] > 0 and c[2][2] > 0, "the collision box must not be empty")
  t.truthy(c[2][1] < 0.5 and c[2][2] < 0.5, "keep the collision box inside one tile (under 0.5 each way)")
end)

t.test("selection_box_covers_it", function()
  local b = box()
  local c, s = b.collision_box, b.selection_box
  t.is_type(s, "table", "add a selection_box")
  mock.validate_prototype(b)
  t.truthy(s[1][1] <= c[1][1] and s[1][2] <= c[1][2], "selection_box must reach past the collision box's top left")
  t.truthy(s[2][1] >= c[2][1] and s[2][2] >= c[2][2], "selection_box must reach past the collision box's bottom right")
end)
