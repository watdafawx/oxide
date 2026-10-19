local t = require("oxide.test")
local s = require("slots")

t.test("length_survives_removal", function()
  local inv = {"iron", "copper", "coal"}
  t.eq(s.remove_slot(inv, 2), "copper", "returns what it removed")
  t.deep_eq(inv, {"iron", "coal"})
  t.eq(#inv, 2)
end)

t.test("counts_trailing_nils_with_select", function()
  t.eq(s.count_args(), 0)
  t.eq(s.count_args(nil), 1)
  t.eq(s.count_args(1, nil, nil), 3)
  t.eq(s.count_args(nil, nil, 3), 3)
end)

t.test("compacts_sparse_list", function()
  t.deep_eq(s.compact({"iron", nil, "coal", nil, "gold"}, 5), {"iron", "coal", "gold"})
  t.deep_eq(s.compact({nil, nil}, 2), {})
  t.deep_eq(s.compact({}, 0), {})
end)

t.test("pack_keeps_nils", function()
  local p = s.pack_all("a", nil, nil)
  t.eq(p.n, 3, "n is the true count")
  t.eq(p[1], "a")
  t.eq(select("#", s.unpack_all(p)), 3, "unpack returns all three")
end)
