local t = require("oxide.test")
local lights = require("lights")

t.test("light_is_green_from_120", function()
  t.eq(lights.light_colour(140), "green")
  t.eq(lights.light_colour(100), "red")
  t.eq(lights.light_colour(120), "green", "exactly 120 cm may ride")
  t.eq(lights.light_colour(119), "red", "119 cm may not")
end)

t.test("photo_price_depends_on_print", function()
  t.eq(lights.photo_price(true), 8, "with a print")
  t.eq(lights.photo_price(false), 3, "download only")
end)
