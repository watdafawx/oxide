local M = {}

-- the price of a plain pizza
function M.base_price()
  local price = 7
  return price
end

-- the base price plus 1.5 for each topping
function M.with_toppings(toppings)
  return price + toppings * 1.5 -- TODO: price does not exist here
end

-- 3 slices for every person at the party
function M.slices_for(people)
  total = 0 -- TODO: this makes a global; keep it inside the function
  for person = 1, people do
    total = total + 3
  end
  return total
end

return M
