-- Kit Mod. Chain: kit-alloy -> kit-crate (item + chest), behind two technologies.

data:extend({
  -- Stage 1 (done): the alloy
  {
    type = "item",
    name = "kit-alloy",
    icon = "__base__/graphics/icons/iron-plate.png",
    stack_size = 100,
    subgroup = "intermediate-product",
    order = "z[kit-alloy]",
  },
  {
    type = "recipe",
    name = "kit-alloy",
    -- TODO (stage 7): lock it until it is researched
    energy_required = 2,
    ingredients = {
      { type = "item", name = "iron-plate", amount = 2 },
      { type = "item", name = "copper-plate", amount = 1 },
    },
    results = { { type = "item", name = "kit-alloy", amount = 1 } },
  },
  {
    type = "technology",
    name = "kit-metallurgy",
    icon = "__base__/graphics/technology/steel-processing.png",
    icon_size = 256,
    -- TODO (stage 6): research automation first (a prerequisite)
    -- TODO (stage 5): effects = unlock-recipe kit-alloy
    -- TODO: unit = 30 units, 15 s, 1 automation-science-pack
  },
})

-- TODO (stage 2): the kit-crate item (stack 50, subgroup "storage", order "a[items]-z[kit-crate]")
--                 and its recipe: 4 kit-alloy + 1 iron-gear-wheel -> 1 kit-crate, locked until researched
-- TODO (stage 3): the kit-crate container: 48 slots, minable, boxes; the item needs place_result
-- TODO (stage 5-6): technology kit-storage after kit-metallurgy: 50 units, 20 s, 1 automation-science-pack
