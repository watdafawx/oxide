local t = require("oxide.test")
local q = require("build_queue")

t.test("adds_to_the_end", function()
  local queue = {"pipe"}
  q.add_job(queue, "iron-stick")
  q.add_job(queue, "copper-cable")
  t.deep_eq(queue, {"pipe", "iron-stick", "copper-cable"})
  t.eq(#queue, 3)
end)

t.test("finishing_closes_the_gap", function()
  local queue = {"pipe", "iron-stick", "copper-cable"}
  t.eq(q.finish_job(queue, 2), "iron-stick", "hands back what it removed")
  t.deep_eq(queue, {"pipe", "copper-cable"}, "the cable moved down")
  t.eq(#queue, 2, "no hole left behind")
end)
