local M = {}

-- under 120 cm: "Too short, sorry"; everyone else: "Enjoy the ride"
function M.gate_message(height)
  -- TODO: the special case for short riders first
  return nil -- TODO: the usual message
end

-- a fast pass costs 10; children under 12 pay 5
function M.fast_pass_price(age)
  local price = 10
  -- TODO: change price for children
  return price
end

return M
