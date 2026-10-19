local t = require("oxide.test")
local reg = require("registry")
local guard = require("guard")

t.test("module_exposes_only_public_api", function()
  local names = {}
  for k in pairs(reg) do names[#names + 1] = k end
  table.sort(names)
  t.deep_eq(names, {"get", "register"}, "keep helpers private with `local`")
  reg.register("gear", {iron = 2})
  t.deep_eq(reg.get("gear"), {iron = 2})
  t.is_nil(reg.get("nope"))
end)

t.test("catches_error_object", function()
  t.is_nil(guard.error_code(function() end), "no error, no code")
  t.eq(guard.error_code(function() error({code = "E_NO_PLATES", need = 3}) end), "E_NO_PLATES")
  t.eq(guard.error_code(function() error("boom") end), "UNKNOWN", "plain string errors")
end)

t.test("rethrows_with_context", function()
  t.eq(guard.with_context("crafting gear", function() return 42 end), 42, "success passes through")
  local function failing()
    guard.with_context("crafting gear", function() error("not enough plates") end)
  end
  t.raises(failing, "crafting gear: ")
  t.raises(failing, "not enough plates")
end)

t.test("finds_accidental_global", function()
  t.deep_eq(guard.find_globals("local a = 1"), {})
  t.deep_eq(guard.find_globals("local a = 1\nb = 2\nc = a"), {"b", "c"})
end)
