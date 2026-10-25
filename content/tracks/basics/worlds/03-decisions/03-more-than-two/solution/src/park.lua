local M = {}

-- under 12: 5. 65 and older: 6. Everyone else: 10.
function M.ticket_price(age)
  if age < 12 then
    return 5
  elseif age >= 65 then
    return 6
  else
    return 10
  end
end

return M
