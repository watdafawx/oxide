local M = {}

-- "iron coal iron" -> {iron = 2, coal = 1}
function M.count_words(text)
  -- TODO: counts = {}; for each word from text:gmatch("%S+"), add 1 to counts[word] (a missing entry is nil: use `or 0`)
  return nil
end

-- delete every entry whose value is 0, in place; return the same table
function M.remove_empty(stock)
  -- TODO: loop with pairs, set zero entries to nil
  return nil
end

-- copy a table and every table inside it
function M.deep_copy(t)
  -- TODO: make out = {}; for each key/value, if type(v) == "table" copy it by calling M.deep_copy(v), else copy v as it is
  return nil
end

return M
