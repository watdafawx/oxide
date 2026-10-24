local M = {}

-- under 120 cm: "Too short, sorry"; everyone else: "Enjoy the ride"
function M.gate_message(height)
  if height < 120 then
    return "Too short, sorry"
  end
  return "Enjoy the ride"
end

-- a fast pass costs 10; children under 12 pay 5
function M.fast_pass_price(age)
  local price = 10
  if age < 12 then
    price = 5
  end
  return price
end

return M
