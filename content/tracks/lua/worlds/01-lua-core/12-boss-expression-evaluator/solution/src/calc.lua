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
      elseif c:match("[%+%-%*/%^]") then
        coroutine.yield({kind = "op", text = c, pos = i})
        i = i + 1
      elseif c == "(" or c == ")" then
        coroutine.yield({kind = c, text = c, pos = i})
        i = i + 1
      elseif c:match("[%a_]") then
        local text = src:match("^[%w_]+", i)
        coroutine.yield({kind = "name", text = text, pos = i})
        i = i + #text
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
    elseif tok.kind == "name" then
      return {type = "var", name = tok.text}
    elseif tok.kind == "(" then
      local inner = expr()
      local close = take()
      if close.kind ~= ")" then
        fail(close.pos, "expected ')' but found " .. describe(close))
      end
      return inner
    end
    fail(tok.pos, "unexpected " .. describe(tok))
  end

  power = function()
    local base = atom()
    local nxt = peek()
    if nxt.kind == "op" and nxt.text == "^" then
      take()
      return {type = "bin", op = "^", left = base, right = unary()}
    end
    return base
  end

  unary = function()
    local nxt = peek()
    if nxt.kind == "op" and nxt.text == "-" then
      take()
      return {type = "neg", expr = unary()}
    end
    return power()
  end

  term = function()
    local left = unary()
    while true do
      local nxt = peek()
      if nxt.kind == "op" and (nxt.text == "*" or nxt.text == "/") then
        take()
        left = {type = "bin", op = nxt.text, left = left, right = unary()}
      else
        return left
      end
    end
  end

  expr = function()
    local left = term()
    while true do
      local nxt = peek()
      if nxt.kind == "op" and (nxt.text == "+" or nxt.text == "-") then
        take()
        left = {type = "bin", op = nxt.text, left = left, right = term()}
      else
        return left
      end
    end
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
  elseif node.type == "neg" then
    return -evaluate(node.expr, env)
  elseif node.type == "var" then
    local value = env[node.name]
    if value == nil then
      error(("unknown variable '%s'"):format(node.name), 0)
    end
    return value
  end
  local a = evaluate(node.left, env)
  local b = evaluate(node.right, env)
  if node.op == "+" then
    return a + b
  elseif node.op == "-" then
    return a - b
  elseif node.op == "*" then
    return a * b
  elseif node.op == "/" then
    if b == 0 then
      error("division by zero", 0)
    end
    return a / b
  end
  return a ^ b
end

function M.eval(src, env)
  return evaluate(M.parse(src), env or {})
end

return M
