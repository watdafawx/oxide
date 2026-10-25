local M = {}

-- 5 -> "====="
function M.underline(width)
  return string.rep("=", width)
end

-- "owls" -> "OWLS"
function M.banner(name)
  return string.upper(name)
end

-- 3 -> "* * *"
function M.stars(n)
  return string.rep("*", n, " ")
end

return M
