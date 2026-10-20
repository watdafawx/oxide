// All sounds are synthesized (WebAudio, no assets). Quiet by design: soft sines, short tails.
import type { Settings, SoundKey } from "./types";

let ctx: AudioContext | null = null;
let master = true;
let volume = 0.6;
let toggles: Record<SoundKey, boolean> = { objective: true, clear: true, level: true, achievement: true, crit: true };

/** the store feeds the learner's settings in here */
export function configureSound(s: Pick<Settings, "sound" | "volume" | "sounds"> | undefined) {
  if (!s) return;
  master = s.sound !== false;
  volume = Math.min(1, Math.max(0, s.volume ?? 0.6));
  toggles = { ...toggles, ...s.sounds };
}

const C5 = 523.25;
const MAJOR = [0, 2, 4, 5, 7, 9, 11];
/** C major scale note `i` steps up from C5 (wraps into the next octave every 7) */
export const scaleNote = (i: number) => C5 * 2 ** ((MAJOR[i % 7] + 12 * Math.floor(i / 7)) / 12);

function audio(): AudioContext | null {
  try {
    ctx ??= new AudioContext();
    if (ctx.state === "suspended") void ctx.resume();
    return ctx;
  } catch {
    return null;
  }
}

interface Note { f: number; at?: number; dur?: number; peak?: number; type?: OscillatorType; to?: number }
function notes(list: Note[], vol: number) {
  const c = audio();
  if (!c || vol <= 0) return;
  try {
    const t0 = c.currentTime;
    for (const n of list) {
      const o = c.createOscillator();
      const g = c.createGain();
      o.type = n.type ?? "sine";
      const t = t0 + (n.at ?? 0);
      const dur = n.dur ?? 0.4;
      o.frequency.setValueAtTime(n.f, t);
      if (n.to) o.frequency.exponentialRampToValueAtTime(n.to, t + dur * 0.8);
      g.gain.setValueAtTime(0.0001, t);
      g.gain.exponentialRampToValueAtTime(Math.max(0.0002, (n.peak ?? 0.3) * vol), t + 0.012);
      g.gain.exponentialRampToValueAtTime(0.0001, t + dur);
      o.connect(g).connect(c.destination);
      o.start(t);
      o.stop(t + dur + 0.05);
    }
  } catch {
    /* audio unavailable: ignore */
  }
}

const SOUNDS: Record<string, (i: number) => Note[]> = {
  /** objective tick: one note up the C major scale per objective index */
  objective: (i) => [{ f: scaleNote(i), dur: 0.28, peak: 0.28, type: "triangle" }],
  /** all green: soft C major chord, a touch of shimmer on top */
  clear: () => [
    { f: scaleNote(0), dur: 0.7, peak: 0.2 },
    { f: scaleNote(2), at: 0.02, dur: 0.7, peak: 0.2 },
    { f: scaleNote(4), at: 0.04, dur: 0.7, peak: 0.2 },
    { f: scaleNote(7), at: 0.06, dur: 0.8, peak: 0.14 },
  ],
  /** level up: rising arpeggio into a held chord */
  level: () => [
    { f: scaleNote(0), dur: 0.3, peak: 0.22 },
    { f: scaleNote(2), at: 0.11, dur: 0.3, peak: 0.22 },
    { f: scaleNote(4), at: 0.22, dur: 0.3, peak: 0.22 },
    { f: scaleNote(7), at: 0.33, dur: 0.9, peak: 0.22 },
    { f: scaleNote(4), at: 0.33, dur: 0.9, peak: 0.16 },
    { f: scaleNote(2), at: 0.33, dur: 0.9, peak: 0.12 },
  ],
  /** achievement sparkle: three quick high pings */
  achievement: () => [
    { f: scaleNote(9), dur: 0.35, peak: 0.16, type: "triangle" },
    { f: scaleNote(11), at: 0.07, dur: 0.35, peak: 0.16, type: "triangle" },
    { f: scaleNote(13), at: 0.14, dur: 0.5, peak: 0.16, type: "triangle" },
    { f: scaleNote(14), at: 0.2, dur: 0.6, peak: 0.1, type: "sine" },
  ],
  /** crit: short upward pitch-bend zap */
  crit: () => [
    { f: 320, to: 1900, dur: 0.16, peak: 0.2, type: "sawtooth" },
    { f: 640, to: 3200, dur: 0.16, peak: 0.08, type: "sine" },
  ],
  /** failed check: a soft low click, never a buzzer */
  fail: () => [{ f: 240, to: 170, dur: 0.07, peak: 0.12, type: "sine" }],
  step: () => [{ f: 220, to: 140, dur: 0.12, peak: 0.1, type: "triangle" }],
};

export type SoundName = keyof typeof SOUNDS;

/** play respecting the master switch, the per-sound toggle and the volume */
export function play(name: SoundName, i = 0) {
  if (!master) return;
  const key = name === "fail" ? "objective" : name === "step" ? null : (name as SoundKey);
  if (key && toggles[key] === false) return;
  notes(SOUNDS[name](i), volume);
}

/** the settings "test" button: ignores the switches, uses the slider's volume */
export function preview(key: SoundKey, vol: number) {
  if (key === "objective") [0, 1, 2].forEach((i) => setTimeout(() => notes(SOUNDS.objective(i), vol), i * 120));
  else notes(SOUNDS[key](0), vol);
}

/** One soft footstep for the map walk. */
export const step = () => play("step");
