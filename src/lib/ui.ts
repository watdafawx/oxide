import type { QuestKind } from "./types";

export const kindLabel: Record<QuestKind, string> = { exercise: "Exercise", boss: "Boss", read: "Read" };
export const kindClass: Record<QuestKind, string> = {
  exercise: "bg-sky-500/15 text-sky-300 border-sky-500/30",
  boss: "bg-rust/15 text-rust-hi border-rust/40",
  read: "bg-violet-500/15 text-violet-300 border-violet-500/30",
};
export const mmss = (s: number) => `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
