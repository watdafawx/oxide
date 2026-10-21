local M = {}

-- pancakes need 3 eggs, omelette 2, cake 4; any other dish needs none
function M.eggs_for(dish)
  if dish == "pancakes" then
    return 3
  elseif dish == "omelette" then
    return 2
  elseif dish == "cake" then
    return 4
  else
    return 0
  end
end

return M
