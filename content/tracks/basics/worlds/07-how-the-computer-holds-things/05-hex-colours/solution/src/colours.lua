local M = {}

-- "FF" -> 255, "88" -> 136, "00" -> 0
function M.hex_byte(text)
  return tonumber(text, 16)
end

-- "#FF8800" -> {255, 136, 0}
function M.to_rgb(code)
  local red = M.hex_byte(string.sub(code, 2, 3))
  local green = M.hex_byte(string.sub(code, 4, 5))
  local blue = M.hex_byte(string.sub(code, 6, 7))
  return {red, green, blue}
end

return M
