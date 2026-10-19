import type { Component } from "svelte";
import type { SceneProps } from "./scene";
import Fallback from "./fallback.svelte";

// one scene per world, file name = the world folder name (e.g. 01-foundations.svelte)
const mods = import.meta.glob<{ default: Component<SceneProps> }>("./[0-9]*.svelte", { eager: true });
const byWorld: Record<string, Component<SceneProps>> = {};
for (const [path, m] of Object.entries(mods)) byWorld[path.slice(2, -".svelte".length)] = m.default;

/** "rust/01-foundations" -> the Foundations scene, else the generic crystal */
export function sceneFor(worldId: string): Component<SceneProps> {
  return byWorld[worldId.split("/").pop() ?? ""] ?? Fallback;
}
export const sceneWorlds = Object.keys(byWorld).sort();
export { Fallback };
export type { SceneProps };
