local M = {}

-- true when there are no free seats left
function M.sold_out(free)
  return free == 0
end

-- true when the visitor may see the late show (15 or older)
function M.old_enough(age)
  return age >= 15
end

return M
