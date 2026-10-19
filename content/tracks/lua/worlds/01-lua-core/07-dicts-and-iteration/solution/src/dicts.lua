local M = {}

-- "iron coal iron" -> {iron = 2, coal = 1}
function M.count_words(text)
  local counts = {}
  for word in text:gmatch("%S+") do
    counts[word] = (counts[word] or 0) + 1
  end
  return counts
end

-- delete every entry whose value is 0, in place; return the same table
function M.remove_empty(stock)
  for item, count in pairs(stock) do
    if count == 0 then
      stock[item] = nil
    end
  end
  return stock
end

-- copy a table and every table inside it
function M.deep_copy(t)
  local out = {}
  for k, v in pairs(t) do
    if type(v) == "table" then
      out[k] = M.deep_copy(v)
    else
      out[k] = v
    end
  end
  return out
end

return M
