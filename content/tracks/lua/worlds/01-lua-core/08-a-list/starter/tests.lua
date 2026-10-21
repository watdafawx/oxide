local t = require("oxide.test")
local book = require("recipe_book")
local cake = {"butter", "sugar", "cocoa", "eggs"}

t.test("first_ingredient_is_item_one", function()
  t.eq(book.first_ingredient(cake), "butter")
  t.eq(book.first_ingredient({"salt"}), "salt")
end)

t.test("counts_the_ingredients", function()
  t.eq(book.ingredient_count(cake), 4)
  t.eq(book.ingredient_count({}), 0)
end)

t.test("last_ingredient_is_at_the_count", function()
  t.eq(book.last_ingredient(cake), "eggs")
  t.eq(book.last_ingredient({"salt"}), "salt")
end)
