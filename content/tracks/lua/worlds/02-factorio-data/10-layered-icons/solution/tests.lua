local t = require("oxide.test")
local mock = require("mock_factorio")

local function item() return mock.load_mod("mod", { validate = false }).item["kit-alloy"] end

t.test("icons_has_two_layers", function()
  local it = item()
  t.is_type(it.icons, "table", "use an `icons` list")
  t.is_nil(it.icon, "with `icons`, the game ignores `icon`: delete that line")
  t.eq(#it.icons, 2, "icons needs exactly two layers")
  t.eq(it.icons[1].icon, "__kit-mod__/graphics/icons/kit-alloy.png", "layer 1 is the Kit Mod's own picture")
  t.eq(it.icons[2].icon, "__base__/graphics/icons/iron-plate.png", "layer 2 is vanilla's iron plate")
end)

t.test("tint_is_a_valid_color", function()
  local it = item()
  t.is_type(it.icons, "table", "use an `icons` list")
  t.is_type(it.icons[2] and it.icons[2].tint, "table", "layer 2 needs a tint")
  mock.validate_prototype(it)   -- rejects broken paths and malformed colours
  local tint = it.icons[2].tint
  local c = { tint.r or tint[1] or 0, tint.g or tint[2] or 0, tint.b or tint[3] or 0 }
  local white = (c[1] == 1 and c[2] == 1 and c[3] == 1) or (c[1] == 255 and c[2] == 255 and c[3] == 255)
  t.falsy(white, "plain white does not tint anything")
end)
