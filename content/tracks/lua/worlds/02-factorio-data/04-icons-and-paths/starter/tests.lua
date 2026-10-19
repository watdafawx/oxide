local t = require("oxide.test")
local mock = require("mock_factorio")

local function item() return mock.load_mod("mod", { validate = false }).item["kit-alloy"] end

t.test("icon_path_uses_mod_prefix", function()
  local it = item()
  t.is_type(it.icons, "table", "use an `icons` list")
  t.eq(it.icons[1].icon, "__kit-mod__/graphics/icons/kit-alloy.png")
end)

t.test("tints_with_icons_layer", function()
  local it = item()
  t.is_type(it.icons, "table", "use an `icons` list")
  t.truthy(#it.icons >= 2, "icons needs a second layer")
  local layer = it.icons[2]
  t.eq(layer.icon, "__base__/graphics/icons/iron-plate.png")
  t.is_type(layer.tint, "table", "layer 2 needs a tint")
end)

t.test("tint_is_a_valid_color", function()
  local it = item()
  t.is_type(it.icons, "table", "use an `icons` list")
  mock.validate_prototype(it)   -- rejects broken paths and malformed colours
  local tint = it.icons[2].tint
  local c = { tint.r or tint[1] or 0, tint.g or tint[2] or 0, tint.b or tint[3] or 0 }
  local white = c[1] == 1 and c[2] == 1 and c[3] == 1
  t.falsy(white, "plain white does not tint anything")
end)
