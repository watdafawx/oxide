local M = {}

-- "energy = 0.5" -> "energy", "0.5"; nil when the line has no "="
function M.parse_key_value(line)
  return line:match("^%s*([%w_%-]+)%s*=%s*(.-)%s*$")
end

-- "iron, copper ,coal" -> {"iron", "copper", "coal"}; skip empty pieces
function M.split_commas(s)
  local out = {}
  for piece in s:gmatch("[^,]+") do
    local item = piece:match("^%s*(.-)%s*$")
    if item ~= "" then
      out[#out + 1] = item
    end
  end
  return out
end

-- "2 iron, 5 copper" -> "4 iron, 10 copper"
function M.double_counts(text)
  return (text:gsub("%d+", function(n) return tonumber(n) * 2 end))
end

-- make s usable as a literal inside a pattern: "a.b" -> "a%.b"
function M.escape_pattern(s)
  return (s:gsub("[%^%$%(%)%%%.%[%]%*%+%-%?]", "%%%0"))
end

return M
