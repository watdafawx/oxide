local M = {}

-- the keypad sends text: "3" -> 3, and "two" -> nil (not a number)
function M.from_keypad(typed)
  return tonumber(typed)
end

-- "4" -> 36: convert the typed text, then 9 per ticket
function M.cost_of_typed(typed)
  return tonumber(typed) * 9
end

return M
