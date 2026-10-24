`kit-crate` can be placed now, but its box is wrong: it starts at the chest's centre and pokes into the next tile, so it would block the space beside it. Boxes are how the game knows how much room an entity takes.

**Measured in tiles.** The map is a grid of tiles, and one tile is 1 unit. A box is `{{left, top}, {right, bottom}}`, measured from the entity's centre. On the map y grows downward, so the top edge is the negative number.

**Centred.** A one-tile chest is centred on its tile, so its box runs the same distance each way: `-0.5` to `0.5` covers the whole tile. A box from `0` to `0.7` starts at the middle and sticks out to the right and down.

**A bit smaller.** `collision_box` is what blocks walking and building. Vanilla chests use `0.35`, a little less than half a tile, so the player can squeeze past and two chests side by side never overlap. Leave it out and the box is empty, so nothing collides with the chest at all.

**What the mouse hits.** `selection_box` is the area your cursor can hover and click. It is usually the whole tile, `-0.5` to `0.5`, so it covers the collision box.

```lua
collision_box = { { -0.4, -0.4 }, { 0.4, 0.4 } },
selection_box = { { -0.5, -0.5 }, { 0.5, 0.5 } },
```

## Your turn

- Centre `collision_box` on 0, a little inside half a tile.
- Add a `selection_box` that covers the whole tile.
