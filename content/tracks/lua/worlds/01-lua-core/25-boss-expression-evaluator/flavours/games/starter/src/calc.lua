local M = {}

-- every syntax problem goes through here: positions count from 1
local function fail(pos, msg)
  error(("syntax error at position %d: %s"):format(pos, msg), 0)
end

local function describe(tok)
  if tok.kind == "eof" then
    return "end of input"
  end
  return "'" .. tok.text .. "'"
end

-- Stage 1: a generator. Each `yield` hands out one token:
--   {kind = "num", text = "3.5", value = 3.5, pos = 6}
local function scan(src)
  return coroutine.wrap(function()
    local i = 1
    while i <= #src do
      local c = src:sub(i, i)
      if c:match("%s") then
        i = i + 1
      elseif c:match("%d") or (c == "." and src:sub(i + 1, i + 1):match("%d")) then
        local text = src:match("^%d*%.?%d*", i)
        coroutine.yield({kind = "num", text = text, value = tonumber(text), pos = i})
        i = i + #text
      -- TODO (stage 1): + - * / ^  ->  yield {kind = "op", text = c, pos = i}, then i = i + 1
      -- TODO (stage 1): ( and )    ->  yield {kind = c, text = c, pos = i}, then i = i + 1
      -- TODO (stage 4): names like iron_plate  ->  yield {kind = "name", text = ..., pos = i}
      else
        fail(i, ("unexpected character '%s'"):format(c))
      end
    end
  end)
end

-- all tokens of src as a list
function M.tokenize(src)
  local list = {}
  for tok in scan(src) do
    list[#list + 1] = tok
  end
  return list
end

-- Stages 2-4: recursive descent. One function per grammar rule:
--   expr  = term  { ("+" | "-") term }
--   term  = unary { ("*" | "/") unary }
--   unary = "-" unary | power
--   power = atom [ "^" unary ]
--   atom  = number | name | "(" expr ")"
-- Nodes: {type = "num", value} {type = "var", name} {type = "neg", expr}
--        {type = "bin", op, left, right}
function M.parse(src)
  local toks = M.tokenize(src)
  toks[#toks + 1] = {kind = "eof", text = "end of input", pos = #src + 1}
  local at = 1
  local function peek() return toks[at] end
  local function take()
    local tok = toks[at]
    at = at + 1
    return tok
  end

  local expr, term, unary, power

  local function atom()
    local tok = take()
    if tok.kind == "num" then
      return {type = "num", value = tok.value}
    end
    -- TODO (stage 4): a "name" token -> {type = "var", name = tok.text}
    -- TODO (stage 3): "(" -> parse expr, then require ")" else fail(close.pos, "expected ')' ...")
    fail(tok.pos, "unexpected " .. describe(tok))
  end

  power = function()
    -- TODO (stage 2): after the atom, if the next token is "^", take it and parse the exponent with unary()
    return atom()
  end

  unary = function()
    -- TODO (stage 3): a "-" token -> {type = "neg", expr = unary()}
    return power()
  end

  term = function()
    -- TODO (stage 2): loop while the next token is "*" or "/"
    return unary()
  end

  expr = function()
    -- TODO (stage 2): loop while the next token is "+" or "-"
    return term()
  end

  local tree = expr()
  local rest = peek()
  if rest.kind ~= "eof" then
    fail(rest.pos, "unexpected " .. describe(rest))
  end
  return tree
end

-- Stages 2, 3, 4, 6: walk the tree and compute the number.
local function evaluate(node, env)
  if node.type == "num" then
    return node.value
  end
  -- TODO (stage 3): "neg"
  -- TODO (stage 4): "var"  (unknown -> error("unknown variable 'x'", 0))
  -- TODO (stage 2): "bin" with + - * / ^
  -- TODO (stage 6): "/" with a zero right side -> error("division by zero", 0)
  return nil
end

function M.eval(src, env)
  return evaluate(M.parse(src), env or {})
end

return M
