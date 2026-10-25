local M = {}

-- already done: 2 for the first 3 km, then 1 per km after that
function M.delivery_fee(km)
  local fee = 2
  if km > 3 then
    fee = 2 + (km - 3)
  end
  return fee
end

-- every pizza costs 7, plus 1.5 for each topping on it
function M.food_price(pizzas, toppings)
  return pizzas * (7 + toppings * 1.5)
end

-- food over 50 gets 10% off (a tenth); otherwise there is no discount
function M.discount(food)
  if food > 50 then
    return food / 10
  end
  return 0
end

-- the food price, minus its discount, plus the delivery fee
function M.order_total(pizzas, toppings, km)
  local food = M.food_price(pizzas, toppings)
  return food - M.discount(food) + M.delivery_fee(km)
end

return M
