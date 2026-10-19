local M = {}

-- true only when v is nil or false
function M.is_falsy(v)
  return v == nil or v == false
end

-- 7 -> 3.5
function M.half(n)
  return n / 2
end

-- 7 -> 3
function M.floor_half(n)
  return math.floor(n / 2)
end

-- 5 -> "5"   (never "5.0")
function M.whole(n)
  return string.format("%d", n)
end

return M
