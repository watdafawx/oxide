import { Marked, type Tokens } from "marked";
import hljs from "highlight.js/lib/core";
import rust from "highlight.js/lib/languages/rust";
import toml from "highlight.js/lib/languages/ini";
import bash from "highlight.js/lib/languages/bash";

hljs.registerLanguage("rust", rust);
hljs.registerLanguage("toml", toml);
hljs.registerLanguage("bash", bash);

const esc = (s: string) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

const md = new Marked({
  gfm: true,
  renderer: {
    code({ text, lang }) {
      const l = (lang ?? "").split(/\s/)[0];
      const html = l && hljs.getLanguage(l) ? hljs.highlight(text, { language: l }).value : esc(text);
      return `<pre><code class="hljs language-${esc(l)}">${html}</code></pre>`;
    },
    link({ href, text }) {
      return `<a href="${esc(href)}" target="_blank" rel="noopener noreferrer">${text}</a>`;
    },
  },
});

export const renderMarkdown = (src: string): string => md.parse(src, { async: false });

const words = (s: string) => (s.match(/\S+/g) ?? []).length;
/** reading time: words / 200, rounded up to the minute (at least 1) */
export const readingMinutes = (src: string): number => Math.max(1, Math.ceil(words(src) / 200));

export interface LessonParts {
  /** the "**Why now:** ..." paragraph, rendered; empty when the lesson has none */
  why: string;
  /** one rendered (inline) html string per bullet of "## Your turn" */
  tasks: string[];
  /** first code block of the idea, rendered */
  code: string;
  /** the rest of the idea, rendered */
  idea: string;
  /** paragraphs + bullets in `idea` */
  points: number;
  /** reading time of `idea` */
  minutes: number;
}

/** Split a lesson into why / task / first code / rest of the idea. null when there is no "## Your turn" list. */
export function splitLesson(src: string): LessonParts | null {
  const toks = md.lexer(src);
  const ti = toks.findIndex((t) => t.type === "heading" && t.depth === 2 && /^your turn$/i.test(t.text.trim()));
  if (ti < 0) return null;
  let end = toks.length;
  for (let i = ti + 1; i < toks.length; i++) {
    const t = toks[i];
    if (t.type === "heading" && t.depth <= 2) { end = i; break; }
  }
  const section = toks.slice(ti + 1, end);
  const list = section.find((t): t is Tokens.List => t.type === "list");
  if (!list || !list.items.length) return null;
  const tasks = list.items.map((it) => md.parseInline(it.text, { async: false }) as string);

  const idea = [...toks.slice(0, ti), ...section.filter((t) => t !== list), ...toks.slice(end)].filter((t) => t.type !== "space");
  let why = "";
  const wi = idea.findIndex((t) => t.type === "paragraph");
  if (wi >= 0 && idea[wi].raw.trimStart().startsWith("**Why now")) {
    why = md.parser([idea[wi]]);
    idea.splice(wi, 1);
  }
  let code = "";
  const ci = idea.findIndex((t) => t.type === "code");
  if (ci >= 0) {
    code = md.parser([idea[ci]]);
    idea.splice(ci, 1);
  }
  let points = 0;
  for (const t of idea) {
    if (t.type === "paragraph") points++;
    else if (t.type === "list") points += t.items.length;
  }
  const raw = idea.map((t) => t.raw).join("");
  return { why, tasks, code, idea: md.parser(idea), points, minutes: readingMinutes(raw) };
}
