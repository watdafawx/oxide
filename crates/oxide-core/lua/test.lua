-- Oxide's Lua test harness: `local t = require("oxide.test")`.
-- One `t.test(name, fn)` per objective. The engine reads `t.__results` after
-- `tests.lua` ran (it calls `t.run()` itself if the file did not).

local M = { __tests = {}, __results = {}, __ran = false, __prefix = "" }

local function sorted_keys(tbl)
  local keys = {}
  for k in pairs(tbl) do keys[#keys + 1] = k end
  table.sort(keys, function(a, b)
    local ta, tb = type(a), type(b)
    if ta == tb and (ta == "number" or ta == "string") then return a < b end
    return ta < tb
  end)
  return keys
end

local function repr(v, depth)
  depth = depth or 0
  local tv = type(v)
  if tv == "string" then return string.format("%q", v) end
  if tv ~= "table" then return tostring(v) end
  if depth > 2 then return "{...}" end
  local parts, n = {}, #v
  for i = 1, n do parts[#parts + 1] = repr(v[i], depth + 1) end
  for _, k in ipairs(sorted_keys(v)) do
    if not (type(k) == "number" and k >= 1 and k <= n and k == math.floor(k)) then
      local ks = type(k) == "string" and k or ("[" .. repr(k, depth + 1) .. "]")
      parts[#parts + 1] = ks .. " = " .. repr(v[k], depth + 1)
    end
  end
  return "{" .. table.concat(parts, ", ") .. "}"
end
M.repr = repr

local function deep_eq(a, b, path)
  if type(a) ~= "table" or type(b) ~= "table" then
    if a == b then return true end
    return false, path
  end
  for k, va in pairs(a) do
    local ok, where = deep_eq(va, b[k], path .. "." .. tostring(k))
    if not ok then return false, where end
  end
  for k in pairs(b) do
    if a[k] == nil then return false, path .. "." .. tostring(k) end
  end
  return true
end

-- Level 3: blame the line in the test body, not this file.
local function fail(msg, prefix)
  if prefix then msg = prefix .. ": " .. msg end
  error(msg, 3)
end

function M.test(name, fn)
  M.__tests[#M.__tests + 1] = { name = M.__prefix .. name, fn = fn }
end

function M.group(prefix, fn)
  local old = M.__prefix
  M.__prefix = old .. prefix
  fn()
  M.__prefix = old
end

function M.eq(actual, expected, msg)
  if actual ~= expected then
    fail(("expected %s, got %s"):format(repr(expected), repr(actual)), msg)
  end
end

function M.ne(a, b, msg)
  if a == b then fail(("expected something other than %s"):format(repr(a)), msg) end
end

function M.deep_eq(actual, expected, msg)
  local ok, where = deep_eq(actual, expected, "value")
  if not ok then
    fail(("expected %s, got %s (differs at %s)"):format(repr(expected), repr(actual), where), msg)
  end
end

function M.truthy(v, msg)
  if not v then fail(("expected a truthy value, got %s"):format(repr(v)), msg) end
end

function M.falsy(v, msg)
  if v then fail(("expected a falsy value, got %s"):format(repr(v)), msg) end
end

function M.is_nil(v, msg)
  if v ~= nil then fail(("expected nil, got %s"):format(repr(v)), msg) end
end

function M.near(a, b, eps, msg)
  eps = eps or 1e-9
  if type(a) ~= "number" or type(b) ~= "number" or math.abs(a - b) > eps then
    fail(("expected %s within %s of %s"):format(repr(a), tostring(eps), repr(b)), msg)
  end
end

function M.is_type(v, tname, msg)
  if type(v) ~= tname then fail(("expected a %s, got %s (%s)"):format(tname, type(v), repr(v)), msg) end
end

function M.contains(haystack, needle, msg)
  if type(haystack) == "string" then
    if not haystack:find(needle, 1, true) then
      fail(("expected %s to contain %s"):format(repr(haystack), repr(needle)), msg)
    end
    return
  end
  if type(haystack) == "table" then
    for _, v in ipairs(haystack) do
      if v == needle then return end
    end
    fail(("expected %s to contain %s"):format(repr(haystack), repr(needle)), msg)
    return
  end
  fail(("contains() needs a string or table, got %s"):format(type(haystack)), msg)
end

function M.raises(fn, pattern, msg)
  local ok, err = pcall(fn)
  if ok then fail("expected an error, none was raised", msg) end
  if pattern and not tostring(err):find(pattern, 1, true) then
    fail(("expected the error to mention %s, got %s"):format(repr(pattern), repr(tostring(err))), msg)
  end
end

function M.skip(reason)
  error({ __skip = reason or "skipped" }, 0)
end

local function trim_traceback(tb)
  local lines = {}
  for line in tostring(tb):gmatch("[^\n]+") do
    if not line:find("oxide/test.lua", 1, true) and not line:find("[C]", 1, true)
      and not line:find("stack traceback", 1, true) then
      lines[#lines + 1] = line:gsub("^%s+", "  ")
    end
    if #lines >= 4 then break end
  end
  return table.concat(lines, "\n")
end

function M.run()
  M.__ran = true
  for _, t in ipairs(M.__tests) do
    local out = {}
    local real_print = print
    print = function(...)
      local parts = {}
      for i = 1, select("#", ...) do parts[#parts + 1] = tostring((select(i, ...))) end
      out[#out + 1] = table.concat(parts, "\t")
    end
    -- No debug.traceback: the sandbox has no debug library. Error messages
    -- already carry `file:line:` of the line that raised.
    local ok, err = xpcall(t.fn, function(e)
      if type(e) == "table" and e.__skip then return e end
      return { msg = tostring(e) }
    end)
    print = real_print
    local r = { name = t.name, out = table.concat(out, "\n") }
    if ok then
      r.status = "ok"
    elseif type(err) == "table" and err.__skip then
      r.status = "ignored"
      r.err = tostring(err.__skip)
    else
      r.status = "FAILED"
      r.err = type(err) == "table" and err.msg or tostring(err)
    end
    M.__results[#M.__results + 1] = r
  end
  return M.__results
end

return M
