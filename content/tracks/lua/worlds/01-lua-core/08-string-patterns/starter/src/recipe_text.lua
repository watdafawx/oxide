local M = {}

-- "energy = 0.5" -> "energy", "0.5"; nil when the line has no "="
function M.parse_key_value(line)
  -- TODO: line:match with two captures
  return nil
end

-- "iron, copper ,coal" -> {"iron", "copper", "coal"}; skip empty pieces
function M.split_commas(s)
  -- TODO: out = {}; for each piece from gmatch "[^,]+", trim spaces off both ends (match the way the lesson trims a value), append it unless it is ""
  return nil
end

-- "2 iron, 5 copper" -> "4 iron, 10 copper"
function M.double_counts(text)
  -- TODO: gsub "%d+" with a function that returns tonumber(n) * 2; wrap the call in ( ) to keep only the string
  return nil
end

-- make s usable as a literal inside a pattern: "a.b" -> "a%.b"
function M.escape_pattern(s)
  -- TODO: gsub with one set [ ] that lists every magic character from the lesson (each escaped with %), replaced by "%" in front of the match
  return nil
end

return M
