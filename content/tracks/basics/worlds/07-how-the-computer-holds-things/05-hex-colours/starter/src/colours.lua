local M = {}

-- "FF" -> 255, "88" -> 136, "00" -> 0
function M.hex_byte(text)
  return nil -- TODO: read text as a hex number
end

-- "#FF8800" -> {255, 136, 0}
function M.to_rgb(code)
  -- TODO: red is characters 2-3, green 4-5, blue 6-7
  return {} -- TODO: a list of the three numbers
end

return M
