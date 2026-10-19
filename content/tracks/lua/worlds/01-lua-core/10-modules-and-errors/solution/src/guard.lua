local M = {}

-- run fn; return the .code of an error table, "UNKNOWN" for other errors, nil if fn succeeds
function M.error_code(fn)
  local ok, err = pcall(fn)
  if ok then
    return nil
  end
  if type(err) == "table" then
    return err.code
  end
  return "UNKNOWN"
end

-- run fn; on error, raise again as "<ctx>: <original message>"; else return fn's result
function M.with_context(ctx, fn)
  local ok, result = pcall(fn)
  if not ok then
    error(ctx .. ": " .. tostring(result), 0)
  end
  return result
end

-- run Lua source in a throwaway environment; return the names it assigned as globals, in order
function M.find_globals(code)
  local found = {}
  local env = setmetatable({}, {
    __newindex = function(_, name)
      found[#found + 1] = name
    end,
  })
  local chunk = assert(load(code, "chunk", "t", env))
  chunk()
  return found
end

return M
