local t = require("oxide.test")
local mock = require("mock_factorio")   -- installs the global `data`, with vanilla items in data.raw
local report = require("stack_report")

t.test("reads_stack_sizes_from_data_raw", function()
  mock.reset()
  t.eq(report.stack_of("iron-plate"), 100, "iron-plate stacks to 100")
  t.eq(report.stack_of("copper-cable"), 200, "copper-cable stacks to 200")
  t.eq(report.stack_of("wooden-chest"), 50, "wooden-chest stacks to 50")
end)

t.test("total_stacks_leaves_no_globals", function()
  mock.reset()
  rawset(_G, "total", nil)
  t.eq(report.total_stacks({ "iron-plate", "copper-cable" }), 300)
  t.is_nil(rawget(_G, "total"), "total_stacks created a global called total: write `local total`")
end)
