local M = {}

-- a pizza is 7, plus 1.5 for each topping
function M.pizza_price(toppings)
  return 7 + toppings * 1.5
end

-- every pizza in the order is the same
function M.order_price(pizzas, toppings)
  return pizzas * (7 + toppings * 1.5)
end

return M
