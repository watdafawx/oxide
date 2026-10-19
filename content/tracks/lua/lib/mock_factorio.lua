-- mock_factorio: a small, honest fake of Factorio 2.0's DATA STAGE for Oxide quests.
--
--   local mock = require("mock_factorio")
--   mock.load_mod("mod")          -- reset, then run mod/data.lua etc. like the game
--   mock.data.raw.item["x"]
--
-- What it fakes (API names and facts: docs/tracks/lua-api-notes.md, Factorio 2.0.55):
--   * globals `data` (`data.raw`, `data:extend`), `mods`, `settings.startup`, `table.deepcopy`,
--     `table_size`, `util` (`util.table.deepcopy`, `util.copy`), `log`, `serpent`
--   * a small vanilla fixture in `data.raw` (plates, gear, circuit, science pack, a few recipes,
--     steel-processing and automation techs): numbers are close to vanilla, not exact
--   * stage order: settings, settings-updates, settings-final-fixes, data, data-updates,
--     data-final-fixes, each round over EVERY mod before the next round starts
--   * `require` inside a mod resolves relative to the mod folder, `__modname__.x` too
--   * mandatory-field validation for item, tool, recipe, technology, fluid, item-subgroup,
--     item-group and container, run at the END of the stage like the game (other types: only
--     `type` and `name` are checked)
-- What it does NOT do: no graphics or file checks (the sandbox cannot see files), no settings
-- prototypes, no locale, no control stage yet (it will live in this table as `mock.load_mod`
-- extensions), no mod dependency ordering (you pass the order), `pairs` order is Lua's.
-- Deviations on purpose: `data:extend` with a duplicate type+name overwrites silently, exactly
-- like the game; see `mock.overwritten()`.

local mock = {}

local real_require = require

local function fail(msg) error(msg, 0) end

local function is_int(v) return type(v) == "number" and v == math.floor(v) end

---------------------------------------------------------------------------
-- Libraries the game provides
---------------------------------------------------------------------------

-- Same behaviour as core/lualib/util.lua: keeps the metatable reference, shares refs, no cycles loop.
local function deepcopy(object)
  local lookup = {}
  local function copy(obj)
    if type(obj) ~= "table" then return obj end
    if lookup[obj] then return lookup[obj] end
    local new = {}
    lookup[obj] = new
    for k, v in pairs(obj) do new[copy(k)] = copy(v) end
    return setmetatable(new, getmetatable(obj))
  end
  return copy(object)
end

local function table_size(t)
  local n = 0
  for _ in pairs(t) do n = n + 1 end
  return n
end

local function sorted_keys(t)
  local keys = {}
  for k in pairs(t) do keys[#keys + 1] = k end
  table.sort(keys, function(a, b)
    if type(a) == type(b) and (type(a) == "number" or type(a) == "string") then return a < b end
    return type(a) < type(b)
  end)
  return keys
end

local function dump(v, indent, multiline)
  if type(v) == "string" then return string.format("%q", v) end
  if type(v) ~= "table" then return tostring(v) end
  local parts = {}
  local pad = multiline and ("\n" .. string.rep("  ", indent + 1)) or ""
  for _, k in ipairs(sorted_keys(v)) do
    local ks = type(k) == "string" and k:match("^[%a_][%w_]*$") and k or ("[" .. dump(k, 0, false) .. "]")
    parts[#parts + 1] = pad .. ks .. " = " .. dump(v[k], indent + 1, multiline)
  end
  local close = multiline and ("\n" .. string.rep("  ", indent)) or ""
  if #parts == 0 then return "{}" end
  return "{" .. table.concat(parts, multiline and "," or ", ") .. close .. "}"
end

local serpent = {
  block = function(v) return dump(v, 0, true) end,
  line = function(v) return dump(v, 0, false) end,
}

local util = { table = { deepcopy = deepcopy } }
util.copy = deepcopy

---------------------------------------------------------------------------
-- Validation: mandatory fields per prototype type (lua-api-notes.md)
---------------------------------------------------------------------------

local function describe(p)
  return ('%s "%s"'):format(tostring(p.type), tostring(p.name))
end

local function bad(p, msg) fail(describe(p) .. ": " .. msg) end

local function need(p, field, kind)
  local v = p[field]
  if v == nil then bad(p, "missing mandatory field `" .. field .. "`") end
  if kind and type(v) ~= kind then
    bad(p, ("field `%s` must be a %s, got %s"):format(field, kind, type(v)))
  end
  return v
end

local function check_color(p, field, c)
  if type(c) ~= "table" then bad(p, ("`%s` must be a colour table like {r = 1, g = 0.5, b = 0}"):format(field)) end
  local named = c.r ~= nil or c.g ~= nil or c.b ~= nil or c.a ~= nil
  local values = named and { c.r or 0, c.g or 0, c.b or 0, c.a or 1 } or c
  if not named and (#c < 3 or #c > 4) then
    bad(p, ("`%s` as an array needs 3 or 4 numbers (r, g, b, a)"):format(field))
  end
  for i = 1, #values do
    if type(values[i]) ~= "number" then bad(p, ("`%s` has a non-number channel"):format(field)) end
  end
end

local function check_path(p, field, path)
  if type(path) ~= "string" then bad(p, ("`%s` must be a string path"):format(field)) end
  if not path:match("^__[%w_%-]+__/") then
    bad(p, ("`%s` = %q must start with a root such as __modname__/ (FileName rule)"):format(field, path))
  end
  if path:find("\\", 1, true) then bad(p, ("`%s` must use forward slashes"):format(field)) end
end

local function check_icons(p)
  if p.icons ~= nil then
    if type(p.icons) ~= "table" or #p.icons == 0 then bad(p, "`icons` cannot be an empty array") end
    for i, layer in ipairs(p.icons) do
      if type(layer) ~= "table" or layer.icon == nil then
        bad(p, ("`icons[%d]` is missing its mandatory field `icon`"):format(i))
      end
      check_path(p, ("icons[%d].icon"):format(i), layer.icon)
      if layer.icon_size ~= nil and not is_int(layer.icon_size) then
        bad(p, ("`icons[%d].icon_size` must be a whole number"):format(i))
      end
      if layer.tint ~= nil then check_color(p, ("icons[%d].tint"):format(i), layer.tint) end
    end
  elseif p.icon == nil then
    bad(p, "missing mandatory field `icon` (or `icons`)")
  else
    check_path(p, "icon", p.icon)
    if p.icon_size ~= nil and not is_int(p.icon_size) then bad(p, "`icon_size` must be a whole number") end
  end
end

local function check_box(p, field)
  local b = p[field]
  if b == nil then return end
  local ok = type(b) == "table" and type(b[1]) == "table" and type(b[2]) == "table"
  if ok then
    for i = 1, 2 do
      ok = ok and type(b[i][1]) == "number" and type(b[i][2]) == "number"
    end
  end
  if not ok then bad(p, ("`%s` must look like {{-0.4, -0.4}, {0.4, 0.4}}"):format(field)) end
end

local function check_amounts(p, list, label, is_result)
  if type(list) ~= "table" then bad(p, ("`%s` must be an array"):format(label)) end
  for i, e in ipairs(list) do
    local where = ("%s %d"):format(label, i)
    if type(e) ~= "table" then bad(p, where .. " must be a table") end
    if e.type == nil then
      if e[1] ~= nil then
        bad(p, where .. ' uses the old {name, amount} form; 2.0 needs {type = "item", name = "...", amount = N}')
      end
      bad(p, where .. " is missing mandatory field `type` (\"item\" or \"fluid\")")
    end
    if e.type ~= "item" and e.type ~= "fluid" then
      bad(p, where .. (' has type %q, expected "item" or "fluid"'):format(tostring(e.type)))
    end
    if type(e.name) ~= "string" then bad(p, where .. " is missing mandatory field `name`") end
    local has_range = e.amount_min ~= nil and e.amount_max ~= nil
    if e.amount == nil and not (is_result and has_range) then
      bad(p, where .. " is missing mandatory field `amount`")
    end
    if e.amount ~= nil then
      if not is_int(e.amount) or e.amount < 1 or e.amount > 65535 then
        bad(p, where .. " `amount` must be a whole number from 1 to 65535")
      end
    end
    if e.probability ~= nil and (type(e.probability) ~= "number" or e.probability < 0 or e.probability > 1) then
      bad(p, where .. " `probability` must be between 0 and 1")
    end
  end
end

local validators = {}

local function check_item(p)
  local size = need(p, "stack_size", "number")
  if not is_int(size) or size < 1 then bad(p, "`stack_size` must be a whole number of at least 1") end
  check_icons(p)
  if p.place_result ~= nil and type(p.place_result) ~= "string" then bad(p, "`place_result` must be an entity name") end
end

validators.item = check_item

validators.tool = function(p)
  check_item(p)
  if not p.infinite then
    local d = need(p, "durability", "number")
    if d <= 0 then bad(p, "`durability` must be positive") end
  end
end

validators.recipe = function(p)
  if p.energy_required ~= nil then
    if type(p.energy_required) ~= "number" or p.energy_required <= 0.001 then
      bad(p, "`energy_required` must be a number greater than 0.001")
    end
  end
  if p.enabled ~= nil and type(p.enabled) ~= "boolean" then bad(p, "`enabled` must be true or false") end
  if p.category ~= nil and type(p.category) ~= "string" then bad(p, "`category` must be a string") end
  if p.ingredients ~= nil then check_amounts(p, p.ingredients, "ingredient", false) end
  if p.results ~= nil then check_amounts(p, p.results, "result", true) end
  if p.main_product ~= nil and type(p.main_product) ~= "string" then bad(p, "`main_product` must be a string") end
  -- The game also needs an icon when the recipe has no single product to borrow one from.
  local n = p.results and #p.results or 0
  if n ~= 1 and not p.main_product and p.icon == nil and p.icons == nil then
    bad(p, "missing mandatory field `icon` (or `icons`): a recipe without exactly one result or a `main_product` has no icon to borrow")
  end
  if p.icon ~= nil or p.icons ~= nil then check_icons(p) end
end

validators.technology = function(p)
  check_icons(p)
  if p.unit == nil and p.research_trigger == nil then
    bad(p, "missing mandatory field `unit` (needed unless `research_trigger` is set)")
  end
  local u = p.unit
  if u ~= nil then
    if type(u) ~= "table" then bad(p, "`unit` must be a table") end
    if u.time == nil then bad(p, "`unit` is missing mandatory field `time`") end
    if type(u.time) ~= "number" or u.time <= 0 then bad(p, "`unit.time` must be a positive number") end
    if u.ingredients == nil then bad(p, "`unit` is missing mandatory field `ingredients`") end
    if u.count == nil and u.count_formula == nil then bad(p, "`unit` needs `count` or `count_formula`") end
    if u.count ~= nil and u.count_formula ~= nil then bad(p, "`unit` has both `count` and `count_formula`, use exactly one") end
    if u.count ~= nil and (not is_int(u.count) or u.count < 1) then bad(p, "`unit.count` must be a whole number above 0") end
    for i, ing in ipairs(u.ingredients) do
      if type(ing) ~= "table" or type(ing[1]) ~= "string" or not is_int(ing[2]) or ing[2] < 1 then
        bad(p, ('`unit.ingredients[%d]` must look like {"automation-science-pack", 1}'):format(i))
      end
    end
  end
  if p.prerequisites ~= nil then
    if type(p.prerequisites) ~= "table" then bad(p, "`prerequisites` must be an array of technology names") end
    for i, name in ipairs(p.prerequisites) do
      if type(name) ~= "string" then bad(p, ("`prerequisites[%d]` must be a technology name"):format(i)) end
    end
  end
  if p.effects ~= nil then
    if type(p.effects) ~= "table" then bad(p, "`effects` must be an array") end
    for i, e in ipairs(p.effects) do
      if type(e) ~= "table" or type(e.type) ~= "string" then
        bad(p, ("`effects[%d]` is missing mandatory field `type`"):format(i))
      end
      if e.type == "unlock-recipe" and type(e.recipe) ~= "string" then
        bad(p, ("`effects[%d]` (unlock-recipe) is missing mandatory field `recipe`"):format(i))
      end
    end
  end
end

validators.fluid = function(p)
  need(p, "default_temperature", "number")
  if p.base_color == nil then bad(p, "missing mandatory field `base_color`") end
  check_color(p, "base_color", p.base_color)
  if p.flow_color == nil then bad(p, "missing mandatory field `flow_color`") end
  check_color(p, "flow_color", p.flow_color)
  check_icons(p)
end

validators["item-subgroup"] = function(p) need(p, "group", "string") end

validators["item-group"] = function(p) check_icons(p) end

local placeable = { ["placeable-neutral"] = true, ["placeable-player"] = true, ["placeable-enemy"] = true }

validators.container = function(p)
  local size = need(p, "inventory_size", "number")
  if not is_int(size) or size < 0 then bad(p, "`inventory_size` must be a whole number") end
  check_box(p, "collision_box")
  check_box(p, "selection_box")
  if p.minable ~= nil then
    if type(p.minable) ~= "table" or p.minable.mining_time == nil then
      bad(p, "`minable` is missing mandatory field `mining_time`")
    end
  end
  for _, flag in ipairs(p.flags or {}) do
    if placeable[flag] then check_icons(p) break end
  end
end

--- Raise an error naming the first missing/invalid field, else return true.
function mock.validate_prototype(p)
  if type(p) ~= "table" then fail("prototype must be a table, got " .. type(p)) end
  if type(p.type) ~= "string" then fail("prototype has no string `type`: " .. serpent.line(p)) end
  if type(p.name) ~= "string" then bad(p, "missing mandatory field `name`") end
  local v = validators[p.type]
  if v then v(p) end
  return true
end

local item_types = { item = true, tool = true, capsule = true }
local entity_types = {
  container = true, furnace = true, ["assembling-machine"] = true, lab = true,
  ["logistic-container"] = true, wall = true, ["transport-belt"] = true,
}

local function find_item(name)
  for t in pairs(item_types) do
    local bucket = mock.data.raw[t]
    if bucket and bucket[name] then return bucket[name] end
  end
end

local function find_entity(name)
  for t in pairs(entity_types) do
    local bucket = mock.data.raw[t]
    if bucket and bucket[name] then return bucket[name] end
  end
end

--- Run `validate_prototype` on everything in `data.raw`; one error listing every problem.
function mock.validate_all()
  local problems = {}
  for _, ty in ipairs(sorted_keys(mock.data.raw)) do
    for _, name in ipairs(sorted_keys(mock.data.raw[ty])) do
      local ok, err = pcall(mock.validate_prototype, mock.data.raw[ty][name])
      if not ok then problems[#problems + 1] = tostring(err) end
    end
  end
  if #problems > 0 then fail(table.concat(problems, "\n")) end
  return true
end

--- Every name that one prototype uses must exist. This is a lint (the game reports unknown
--- names when it loads the prototypes), listing every unresolved reference in one error.
function mock.validate_references()
  local raw = mock.data.raw
  local problems = {}
  local function add(fmt, ...) problems[#problems + 1] = fmt:format(...) end
  for _, name in ipairs(sorted_keys(raw.recipe or {})) do
    local r = raw.recipe[name]
    for _, e in ipairs(r.ingredients or {}) do
      local found = e.type == "fluid" and raw.fluid and raw.fluid[e.name] or find_item(e.name)
      if not found then add('recipe "%s": ingredient "%s" does not exist', name, tostring(e.name)) end
    end
    for _, e in ipairs(r.results or {}) do
      local found = e.type == "fluid" and raw.fluid and raw.fluid[e.name] or find_item(e.name)
      if not found then add('recipe "%s": result "%s" does not exist', name, tostring(e.name)) end
    end
    if r.main_product and r.main_product ~= "" then
      local listed = false
      for _, e in ipairs(r.results or {}) do listed = listed or e.name == r.main_product end
      if not listed then add('recipe "%s": main_product "%s" is not one of its results', name, r.main_product) end
    end
  end
  for _, name in ipairs(sorted_keys(raw.technology or {})) do
    local t = raw.technology[name]
    for _, pre in ipairs(t.prerequisites or {}) do
      if not (raw.technology and raw.technology[pre]) then
        add('technology "%s": prerequisite "%s" does not exist', name, pre)
      end
    end
    for _, e in ipairs(t.effects or {}) do
      if e.type == "unlock-recipe" and not (raw.recipe and raw.recipe[e.recipe]) then
        add('technology "%s": unlock-recipe "%s" does not exist', name, tostring(e.recipe))
      end
    end
    for _, ing in ipairs(t.unit and t.unit.ingredients or {}) do
      local tool = raw.tool and raw.tool[ing[1]]
      if not tool then add('technology "%s": research ingredient "%s" is not a tool item', name, tostring(ing[1])) end
    end
  end
  for ty in pairs(item_types) do
    for _, name in ipairs(sorted_keys(raw[ty] or {})) do
      local it = raw[ty][name]
      if it.place_result and it.place_result ~= "" and not find_entity(it.place_result) then
        add('%s "%s": place_result "%s" is not an entity prototype', ty, name, it.place_result)
      end
    end
  end
  for ty in pairs(entity_types) do
    for _, name in ipairs(sorted_keys(raw[ty] or {})) do
      local m = raw[ty][name].minable
      if m and m.result and not find_item(m.result) then
        add('%s "%s": minable result "%s" is not an item', ty, name, m.result)
      end
    end
  end
  if #problems > 0 then fail(table.concat(problems, "\n")) end
  return true
end

---------------------------------------------------------------------------
-- The vanilla fixture
---------------------------------------------------------------------------

local function icon_of(name) return "__base__/graphics/icons/" .. name .. ".png" end

local function fixture()
  local raw = {
    ["item-group"] = {
      logistics = { type = "item-group", name = "logistics", icon = icon_of("wooden-chest"), order = "a" },
      production = { type = "item-group", name = "production", icon = icon_of("assembling-machine-1"), order = "c" },
      ["intermediate-products"] = {
        type = "item-group", name = "intermediate-products", icon = icon_of("iron-gear-wheel"), order = "d",
      },
    },
    ["item-subgroup"] = {
      storage = { type = "item-subgroup", name = "storage", group = "logistics", order = "a" },
      ["raw-material"] = { type = "item-subgroup", name = "raw-material", group = "intermediate-products", order = "b" },
      ["intermediate-product"] = {
        type = "item-subgroup", name = "intermediate-product", group = "intermediate-products", order = "c",
      },
      ["science-pack"] = { type = "item-subgroup", name = "science-pack", group = "intermediate-products", order = "i" },
      ["production-machine"] = {
        type = "item-subgroup", name = "production-machine", group = "production", order = "e",
      },
    },
    item = {}, tool = {}, recipe = {}, technology = {}, container = {}, furnace = {},
    ["assembling-machine"] = {}, lab = {},
  }
  local function item(name, stack, subgroup, order, extra)
    local it = {
      type = "item", name = name, icon = icon_of(name), stack_size = stack, subgroup = subgroup, order = order,
    }
    for k, v in pairs(extra or {}) do it[k] = v end
    raw.item[name] = it
  end
  item("iron-plate", 100, "raw-material", "b[iron-plate]")
  item("copper-plate", 100, "raw-material", "c[copper-plate]")
  item("steel-plate", 100, "raw-material", "d[steel-plate]")
  item("copper-cable", 200, "intermediate-product", "a[copper-cable]")
  item("iron-gear-wheel", 100, "intermediate-product", "c[iron-gear-wheel]")
  item("electronic-circuit", 200, "intermediate-product", "b[circuits]-a[electronic-circuit]")
  item("wooden-chest", 50, "storage", "a[items]-a[wooden-chest]", { place_result = "wooden-chest" })
  item("stone-furnace", 50, "production-machine", "a[smelting]-a[stone-furnace]", { place_result = "stone-furnace" })
  item("assembling-machine-1", 50, "production-machine", "a[assembling-machine-1]",
    { place_result = "assembling-machine-1" })
  item("lab", 50, "production-machine", "g[lab]", { place_result = "lab" })

  raw.tool["automation-science-pack"] = {
    type = "tool", name = "automation-science-pack", icon = icon_of("automation-science-pack"),
    stack_size = 200, durability = 1, subgroup = "science-pack", order = "a[automation-science-pack]",
  }

  raw.container["wooden-chest"] = {
    type = "container", name = "wooden-chest", icon = icon_of("wooden-chest"), inventory_size = 16,
    flags = { "placeable-neutral", "player-creation" },
    minable = { mining_time = 0.2, result = "wooden-chest" },
    collision_box = { { -0.35, -0.35 }, { 0.35, 0.35 } }, selection_box = { { -0.5, -0.5 }, { 0.5, 0.5 } },
  }
  raw.furnace["stone-furnace"] = { type = "furnace", name = "stone-furnace", icon = icon_of("stone-furnace") }
  raw["assembling-machine"]["assembling-machine-1"] = {
    type = "assembling-machine", name = "assembling-machine-1", icon = icon_of("assembling-machine-1"),
  }
  raw.lab["lab"] = { type = "lab", name = "lab", icon = icon_of("lab") }

  local function recipe(name, ingredients, results, energy, enabled)
    local r = { type = "recipe", name = name, energy_required = energy, ingredients = {}, results = results }
    for _, pair in ipairs(ingredients) do
      r.ingredients[#r.ingredients + 1] = { type = "item", name = pair[1], amount = pair[2] }
    end
    if enabled == false then r.enabled = false end
    raw.recipe[name] = r
  end
  local function result(name, n) return { { type = "item", name = name, amount = n } } end
  recipe("iron-gear-wheel", { { "iron-plate", 2 } }, result("iron-gear-wheel", 1), 0.5)
  recipe("copper-cable", { { "copper-plate", 1 } }, result("copper-cable", 2), 0.5)
  recipe("electronic-circuit", { { "iron-plate", 1 }, { "copper-cable", 3 } }, result("electronic-circuit", 1), 0.5)
  recipe("automation-science-pack", { { "copper-plate", 1 }, { "iron-gear-wheel", 1 } },
    result("automation-science-pack", 1), 5)
  recipe("steel-plate", { { "iron-plate", 5 } }, result("steel-plate", 1), 16, false)
  recipe("assembling-machine-1",
    { { "electronic-circuit", 3 }, { "iron-gear-wheel", 5 }, { "iron-plate", 9 } },
    result("assembling-machine-1", 1), 0.5, false)

  raw.technology["automation"] = {
    type = "technology", name = "automation", icon = "__base__/graphics/technology/automation.png", icon_size = 256,
    effects = { { type = "unlock-recipe", recipe = "assembling-machine-1" } },
    unit = { count = 10, ingredients = { { "automation-science-pack", 1 } }, time = 10 },
  }
  raw.technology["steel-processing"] = {
    type = "technology", name = "steel-processing", icon = "__base__/graphics/technology/steel-processing.png",
    icon_size = 256,
    effects = { { type = "unlock-recipe", recipe = "steel-plate" } },
    unit = { count = 50, ingredients = { { "automation-science-pack", 1 } }, time = 5 },
  }
  return raw
end

--- A fresh copy of the untouched vanilla fixture, for "original is untouched" checks.
function mock.pristine() return fixture() end

---------------------------------------------------------------------------
-- Globals: data, mods, settings
---------------------------------------------------------------------------

local data = {}
local overwrites = {}
local startup = {}

-- `data:extend` as in core/lualib/dataloader.lua: array of prototypes, type and name must be strings,
-- the last definition of a type+name wins.
function data.extend(self, otherdata)
  if self ~= data and otherdata == nil then otherdata = self end
  if type(otherdata) ~= "table" then
    fail("Invalid array of prototypes:\n\n" .. serpent.line(otherdata))
  end
  if #otherdata == 0 then
    if otherdata.type or otherdata.name then
      fail("Expected array of prototypes, but got a single prototype:\n\n" .. serpent.block(otherdata))
    end
    fail("Invalid array of prototypes (it is empty)")
  end
  for _, e in ipairs(otherdata) do
    if type(e) ~= "table" or type(e.type) ~= "string" then
      fail("Invalid type in the following prototype definition:\n" .. serpent.block(e))
    end
    if type(e.name) ~= "string" then
      fail("Invalid name in the following prototype definition:\n" .. serpent.block(e))
    end
    local bucket = data.raw[e.type]
    if bucket == nil then
      bucket = {}
      data.raw[e.type] = bucket
    end
    if bucket[e.name] ~= nil then overwrites[#overwrites + 1] = e.type .. "/" .. e.name end
    bucket[e.name] = e
  end
end

mock.data = data
mock.stage_log = {}

--- `type/name` of every prototype that `data:extend` replaced, in order.
function mock.overwritten()
  return deepcopy(overwrites)
end

local mods = {}
local settings = {
  startup = setmetatable({}, {
    __index = function(_, k)
      if startup[k] == nil then return nil end
      return { value = startup[k] }
    end,
    __newindex = function() fail("startup settings are read-only in the data stage") end,
  }),
}

--- Set the value `settings.startup[name].value` returns.
function mock.set_startup_setting(name, value) startup[name] = value end

function mock.reset()
  data.raw = fixture()
  overwrites = {}
  startup = {}
  mock.stage_log = {}
  for k in pairs(mods) do mods[k] = nil end
  mods.base = "2.0.55"
  mods.core = "2.0.55"
end

---------------------------------------------------------------------------
-- Running a mod
---------------------------------------------------------------------------

local STAGES = {
  "settings", "settings-updates", "settings-final-fixes",
  "data", "data-updates", "data-final-fixes",
}

local mod_dirs = {}     -- mod name -> folder (as a require prefix)
local current_dir       -- folder of the mod whose file is running

local function not_found(err, modname)
  return type(err) == "string" and err:find("module '" .. modname .. "' not found", 1, true) ~= nil
end

local function basename(dir) return (dir:gsub("[/\\]", "."):match("([^.]+)$")) end

-- Inside a mod, `require("prototypes.item")` is relative to the mod folder and
-- `require("__mymod__.prototypes.item")` names the mod; `util` and friends fall through.
local function mod_require(name)
  local mod, rest = name:match("^__([%w_%-]+)__%.(.+)$")
  if mod and mod_dirs[mod] then return real_require(mod_dirs[mod] .. "." .. rest) end
  if current_dir and not name:find("^__") then
    local ok, res = pcall(real_require, current_dir .. "." .. name)
    if ok then return res end
    if not not_found(res, current_dir .. "." .. name) then error(res, 0) end
  end
  return real_require(name)
end

local function clear_loaded(dir)
  for k in pairs(package.loaded) do
    if k:sub(1, #dir + 1) == dir .. "." then package.loaded[k] = nil end
  end
end

--- Load several mods in the order given: reset, then every stage round over every mod.
--- opts: `keep = true` skips the reset, `validate = false` skips the end-of-stage validation.
function mock.load_mods(dirs, opts)
  if not (opts and opts.keep) then mock.reset() end
  mock.stage_log = {}
  for _, dir in ipairs(dirs) do
    dir = dir:gsub("[/\\]", ".")
    mod_dirs[basename(dir)] = dir
    mods[basename(dir)] = "0.0.1"
    clear_loaded(dir)
  end
  for _, stage in ipairs(STAGES) do
    for _, dir in ipairs(dirs) do
      dir = dir:gsub("[/\\]", ".")
      local modname = dir .. "." .. stage
      package.loaded[modname] = nil
      current_dir = dir
      local ok, err = pcall(real_require, modname)
      current_dir = nil
      if ok then
        mock.stage_log[#mock.stage_log + 1] = basename(dir) .. ":" .. stage
      elseif not not_found(err, modname) then
        error(err, 0)
      end
    end
  end
  if not (opts and opts.validate == false) then mock.validate_all() end
  return data.raw
end

function mock.load_mod(dir, opts) return mock.load_mods({ dir }, opts) end

---------------------------------------------------------------------------
-- info.json helpers (Oxide cannot read files yet, so tests hand over the text)
---------------------------------------------------------------------------

--- Tiny JSON decoder: objects, arrays, strings, numbers, true/false/null (null becomes nil).
function mock.decode_json(text)
  local pos = 1
  local function err(msg) fail(("JSON error at character %d: %s"):format(pos, msg)) end
  local function skip() pos = text:find("[^ \t\r\n]", pos) or #text + 1 end
  local value
  local function str()
    local out = {}
    pos = pos + 1
    while true do
      local c = text:sub(pos, pos)
      if c == "" then err("unterminated string") end
      if c == '"' then pos = pos + 1 return table.concat(out) end
      if c == "\\" then
        local e = text:sub(pos + 1, pos + 1)
        local map = { n = "\n", t = "\t", r = "\r", b = "\b", f = "\f", ['"'] = '"', ["\\"] = "\\", ["/"] = "/" }
        if e == "u" then
          local code = tonumber(text:sub(pos + 2, pos + 5), 16) or err("bad \\u escape")
          out[#out + 1] = code < 128 and string.char(code) or "?"
          pos = pos + 6
        elseif map[e] then
          out[#out + 1] = map[e]
          pos = pos + 2
        else
          err("bad escape \\" .. e)
        end
      else
        out[#out + 1] = c
        pos = pos + 1
      end
    end
  end
  function value()
    skip()
    local c = text:sub(pos, pos)
    if c == "{" then
      local obj = {}
      pos = pos + 1
      skip()
      if text:sub(pos, pos) == "}" then pos = pos + 1 return obj end
      while true do
        skip()
        if text:sub(pos, pos) ~= '"' then err("expected a string key") end
        local k = str()
        skip()
        if text:sub(pos, pos) ~= ":" then err("expected ':' after key") end
        pos = pos + 1
        obj[k] = value()
        skip()
        local d = text:sub(pos, pos)
        pos = pos + 1
        if d == "}" then return obj end
        if d ~= "," then pos = pos - 1 err("expected ',' or '}'") end
      end
    elseif c == "[" then
      local arr = {}
      pos = pos + 1
      skip()
      if text:sub(pos, pos) == "]" then pos = pos + 1 return arr end
      while true do
        arr[#arr + 1] = value()
        skip()
        local d = text:sub(pos, pos)
        pos = pos + 1
        if d == "]" then return arr end
        if d ~= "," then pos = pos - 1 err("expected ',' or ']'") end
      end
    elseif c == '"' then
      return str()
    elseif text:sub(pos, pos + 3) == "true" then
      pos = pos + 4 return true
    elseif text:sub(pos, pos + 4) == "false" then
      pos = pos + 5 return false
    elseif text:sub(pos, pos + 3) == "null" then
      pos = pos + 4 return nil
    else
      local num = text:match("^-?%d+%.?%d*[eE]?[+-]?%d*", pos)
      if not num or num == "" then err("unexpected character '" .. c .. "'") end
      pos = pos + #num
      return tonumber(num) or err("bad number " .. num)
    end
  end
  local result = value()
  skip()
  if pos <= #text then err("extra text after the JSON value") end
  return result
end

--- Problems with an info.json table (decoded), as a list of strings; empty means clean.
function mock.lint_info(info)
  local problems = {}
  local function add(s) problems[#problems + 1] = s end
  if type(info) ~= "table" then return { "info.json must be a JSON object" } end
  for _, f in ipairs({ "name", "version", "title", "author" }) do
    if type(info[f]) ~= "string" or info[f] == "" then add("missing mandatory field `" .. f .. "`") end
  end
  if type(info.name) == "string" and info.name ~= "" then
    if not info.name:match("^[%w_%-]+$") then add("`name` may only use letters, digits, - and _ (portal rule)") end
    if #info.name < 4 or #info.name > 49 then add("`name` must be 4 to 49 characters (portal rule)") end
  end
  if type(info.version) == "string" then
    local a, b, c = info.version:match("^(%d+)%.(%d+)%.(%d+)$")
    if not a then
      add("`version` must look like 1.0.0 (three numbers)")
    elseif tonumber(a) > 65535 or tonumber(b) > 65535 or tonumber(c) > 65535 then
      add("each number in `version` must be at most 65535")
    end
  end
  if info.factorio_version == nil then
    add("missing `factorio_version` (the default is the ancient \"0.12\")")
  elseif type(info.factorio_version) ~= "string" or not info.factorio_version:match("^%d+%.%d+$") then
    add("`factorio_version` must be two numbers like \"2.0\"")
  end
  if info.dependencies ~= nil then
    if type(info.dependencies) ~= "table" then
      add("`dependencies` must be an array of strings")
    else
      for i, dep in ipairs(info.dependencies) do
        local s = tostring(dep)
        s = s:gsub("^%(%?%)%s*", ""):gsub("^[!?+~]%s*", "")
        local name, rest = s:match("^([%w_%-]+)%s*(.*)$")
        local ok = name ~= nil
        if ok and rest ~= "" then
          ok = rest:match("^[<>]=?%s*%d+%.%d+%.%d+$") ~= nil or rest:match("^=%s*%d+%.%d+%.%d+$") ~= nil
        end
        if not ok then add(("dependency %d (%q) is not `[prefix] name [op version]`"):format(i, tostring(dep))) end
      end
    end
  end
  return problems
end

---------------------------------------------------------------------------
-- Install the globals
---------------------------------------------------------------------------

mock.logged = {}
_G.data = data
_G.mods = mods
_G.settings = settings
_G.util = util
_G.serpent = serpent
_G.table_size = table_size
_G.log = function(msg) mock.logged[#mock.logged + 1] = tostring(msg) end
table.deepcopy = deepcopy
package.preload["util"] = function() return util end
_G.require = mod_require

mock.reset()
return mock
