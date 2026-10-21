local M = {}

-- gear 2 iron plates, pipe 1, steel 5; any other item needs none
function M.plates_for(item)
  if item == "iron-gear-wheel" then
    return 2
  elseif item == "pipe" then
    return 1
  elseif item == "steel-plate" then
    return 5
  else
    return 0
  end
end

return M
