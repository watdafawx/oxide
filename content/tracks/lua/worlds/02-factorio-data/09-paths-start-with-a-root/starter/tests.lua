local t = require("oxide.test")
local mock = require("mock_factorio")

t.test("icon_path_uses_mod_root", function()
  local item = mock.load_mod("mod", { validate = false }).item["kit-alloy"]
  mock.validate_prototype(item)   -- rejects a path without a root
  t.eq(item.icon, "__kit-mod__/graphics/icons/kit-alloy.png")
end)
