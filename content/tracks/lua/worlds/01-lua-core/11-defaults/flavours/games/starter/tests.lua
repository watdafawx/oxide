local t = require("oxide.test")
local d = require("defaults")

t.test("missing_crafts_means_one", function()
  t.eq(d.crafts_wanted(), 1)
  t.eq(d.crafts_wanted(nil), 1)
  t.eq(d.crafts_wanted(4), 4)
end)

t.test("false_is_a_real_answer", function()
  t.eq(d.option(false, true), false, "false must survive")
  t.eq(d.option(nil, true), true)
  t.eq(d.option(0, 5), 0, "0 is a real value too")
end)
