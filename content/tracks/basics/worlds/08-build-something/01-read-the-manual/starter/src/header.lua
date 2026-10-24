local M = {}

-- 5 -> "====="
function M.underline(width)
  return "" -- TODO: string.rep, see the manual entry
end

-- "owls" -> "OWLS"
function M.banner(name)
  return name -- TODO: string.upper, see the manual entry
end

-- 3 -> "* * *"
function M.stars(n)
  return "" -- TODO: string.rep with its optional argument
end

return M
