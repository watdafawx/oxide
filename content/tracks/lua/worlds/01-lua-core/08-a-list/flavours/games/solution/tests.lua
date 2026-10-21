local t = require("oxide.test")
local book = require("recipe_book")
local engine = {"steel-plate", "iron-gear-wheel", "pipe", "iron-plate"}

t.test("first_ingredient_is_item_one", function()
  t.eq(book.first_ingredient(engine), "steel-plate")
  t.eq(book.first_ingredient({"pipe"}), "pipe")
end)

t.test("counts_the_ingredients", function()
  t.eq(book.ingredient_count(engine), 4)
  t.eq(book.ingredient_count({}), 0)
end)

t.test("last_ingredient_is_at_the_count", function()
  t.eq(book.last_ingredient(engine), "iron-plate")
  t.eq(book.last_ingredient({"pipe"}), "pipe")
end)
