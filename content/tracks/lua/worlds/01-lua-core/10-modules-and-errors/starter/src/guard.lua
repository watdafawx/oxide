local M = {}

-- run fn; return the .code of an error table, "UNKNOWN" for other errors, nil if fn succeeds
function M.error_code(fn)
  -- TODO: pcall, then look at the error value
  return nil
end

-- run fn; on error, raise again as "<ctx>: <original message>"; else return fn's result
function M.with_context(ctx, fn)
  -- TODO: pcall + error(..., 0)
  return nil
end

-- run Lua source in a throwaway environment; return the names it assigned as globals, in order
function M.find_globals(code)
  -- TODO: load(code, "chunk", "t", env) with an env whose __newindex records the name
  return nil
end

return M
