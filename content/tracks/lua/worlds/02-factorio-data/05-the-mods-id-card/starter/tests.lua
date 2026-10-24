local t = require("oxide.test")
local mock = require("mock_factorio")
local text = require("info_json")

local function info() return mock.decode_json(text) end

t.test("has_required_fields", function()
  local i = info()
  for _, field in ipairs({"name", "version", "title", "author"}) do
    t.is_type(i[field], "string", field .. " must be a string")
    t.ne(i[field], "", field .. " is empty")
  end
  t.eq(i.name, "kit-mod")
  t.truthy(i.version:match("^%d+%.%d+%.%d+$"), "version needs three numbers like 0.1.0")
end)

t.test("factorio_version_is_2_0", function()
  t.eq(info().factorio_version, "2.0")
end)

t.test("depends_on_base", function()
  local i = info()
  t.is_type(i.dependencies, "table")
  local found = false
  for _, dep in ipairs(i.dependencies) do
    if dep == "base" or dep:match("^base%s") then found = true end
  end
  t.truthy(found, "dependencies must contain a `base ...` entry")
  t.deep_eq(mock.lint_info(i), {}, "info.json problems")
end)
