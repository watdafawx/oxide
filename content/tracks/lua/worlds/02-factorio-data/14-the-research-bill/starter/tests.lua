local t = require("oxide.test")
local mock = require("mock_factorio")

local function tech() return mock.load_mod("mod", { validate = false }).technology["kit-metallurgy"] end

t.test("unit_costs_red_science", function()
  local u = tech().unit
  t.is_type(u, "table", "add a `unit`")
  t.eq(u.count, 30, "count")
  t.eq(u.time, 15, "time per unit")
  t.deep_eq(u.ingredients, { { "automation-science-pack", 1 } })
end)

t.test("tech_passes_the_field_check", function()
  mock.validate_prototype(tech())   -- names the missing or malformed field
  mock.validate_references()        -- research ingredients must be tool items
end)
