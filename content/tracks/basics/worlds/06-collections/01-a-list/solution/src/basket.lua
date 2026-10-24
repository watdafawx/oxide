local M = {}

-- {2.5, 1, 4} -> 2.5
function M.first_item(basket)
  return basket[1]
end

-- {2.5, 1, 4} -> 3
function M.item_count(basket)
  return #basket
end

-- {2.5, 1, 4} -> 1: the one just before the last
function M.second_to_last(basket)
  return basket[#basket - 1]
end

return M
