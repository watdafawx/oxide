import type { ObjectiveResult, Quest } from "../types";

export interface SceneProps { quest: Quest; objectives: ObjectiveResult[] }

/** Count-based progress: n of N objectives passed. */
export function prog(objs: ObjectiveResult[]) {
  const total = objs.length;
  const n = objs.filter((o) => o.status === "passed").length;
  return { n, total, frac: total ? n / total : 0, complete: total > 0 && n === total };
}

/** Is stage k (1-based) of S lit at this progress? Full progress lights every stage. */
export const lit = (frac: number, k: number, S: number) => frac * S >= k - 1e-9;

let nextId = 0;
export const uid = (p: string) => `${p}${++nextId}`;
