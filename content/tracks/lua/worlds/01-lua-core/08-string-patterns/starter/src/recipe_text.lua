local M = {}

-- "energy = 0.5" -> "energy", "0.5"; nil when the line has no "="
function M.parse_key_value(line)
  -- TODO: line:match with two captures
  return nil
end

-- "iron, copper ,coal" -> {"iron", "copper", "coal"}; skip empty pieces
function M.split_commas(s)
  -- TODO: gmatch "[^,]+", then trim each piece
  return nil
end

-- "2 iron, 5 copper" -> "4 iron, 10 copper"
function M.double_counts(text)
  -- TODO: gsub with a function; keep only the string result
  return nil
end

-- make s usable as a literal inside a pattern: "a.b" -> "a%.b"
function M.escape_pattern(s)
  -- TODO: gsub every magic character -> "%" .. char
  return nil
end

return M
