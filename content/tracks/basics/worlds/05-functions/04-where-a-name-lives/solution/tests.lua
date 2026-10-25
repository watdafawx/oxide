local t = require("oxide.test")
local kitchen = require("kitchen")

t.test("toppings_add_to_the_base_price", function()
  t.eq(kitchen.with_toppings(0), 7, "no toppings")
  t.eq(kitchen.with_toppings(2), 10, "two toppings")
end)

t.test("slices_leave_no_global_behind", function()
  t.eq(kitchen.slices_for(4), 12, "4 people, 3 slices each")
  -- rawget(_G, "total") looks for a global called total
  t.is_nil(rawget(_G, "total"), "slices_for left a global called total behind")
end)
