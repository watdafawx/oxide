local t = require("oxide.test")
local registry = require("registry")

t.test("module_exposes_only_register_and_get", function()
  local names = {}
  for key in pairs(registry) do names[#names + 1] = key end
  table.sort(names)
  t.deep_eq(names, {"get", "register"}, "keep the table private with `local`")
end)

t.test("registers_and_finds_a_recipe", function()
  registry.register("iron-gear-wheel", {["iron-plate"] = 2})
  t.deep_eq(registry.get("iron-gear-wheel"), {["iron-plate"] = 2})
  t.is_nil(registry.get("nope"), "an unknown name gives nil")
end)
