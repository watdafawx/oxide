local M = {}

-- "iron coal iron" -> {iron = 2, coal = 1}
function M.count_words(text)
  -- TODO: gmatch over words, bump a counter per word
  return nil
end

-- delete every entry whose value is 0, in place; return the same table
function M.remove_empty(stock)
  -- TODO: loop with pairs, set zero entries to nil
  return nil
end

-- copy a table and every table inside it
function M.deep_copy(t)
  -- TODO: recurse into values that are tables
  return nil
end

return M
