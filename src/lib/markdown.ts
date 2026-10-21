import { Marked, type Token, type Tokens } from "marked";
import hljs from "highlight.js/lib/core";
import rust from "highlight.js/lib/languages/rust";
import toml from "highlight.js/lib/languages/ini";
import bash from "highlight.js/lib/languages/bash";
import java from "highlight.js/lib/languages/java";
import lua from "highlight.js/lib/languages/lua";

hljs.registerLanguage("rust", rust);
hljs.registerLanguage("toml", toml);
hljs.registerLanguage("bash", bash);
hljs.registerLanguage("java", java);
hljs.registerLanguage("lua", lua);

/** highlight language for inline `code` spans while a render call is running (set by the exported functions) */
let curLang = "";
const unesc = (s: string) => s.replace(/&lt;/g, "<").replace(/&gt;/g, ">").replace(/&quot;/g, '"').replace(/&#39;/g, "'").replace(/&amp;/g, "&");
const withLang = <T>(lang: string | undefined, fn: () => T): T => {
  const prev = curLang;
  curLang = lang ?? "";
  try { return fn(); } finally { curLang = prev; }
};
/** the highlight language for a runner */
export const langOf = (runner: string | undefined): string => (runner === "cargo" ? "rust" : runner === "gradle" ? "java" : runner === "lua" ? "lua" : "");

const esc = (s: string) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

const md = new Marked({
  gfm: true,
  renderer: {
    code({ text, lang }) {
      const l = (lang ?? "").split(/\s/)[0];
      const html = l && hljs.getLanguage(l) ? hljs.highlight(text, { language: l }).value : esc(text);
      return `<pre><code class="hljs language-${esc(l)}">${html}</code></pre>`;
    },
    codespan({ text }) {
      const raw = unesc(text);
      if (curLang && hljs.getLanguage(curLang)) {
        try {
          const h = hljs.highlight(raw, { language: curLang, ignoreIllegals: true }).value;
          if (h.includes("hljs-")) return `<code class="hljs">${h}</code>`;
        } catch { /* plain below */ }
      }
      return `<code>${esc(raw)}</code>`;
    },
    link({ href, text }) {
      return `<a href="${esc(href)}" target="_blank" rel="noopener noreferrer">${text}</a>`;
    },
  },
});

export const renderMarkdown = (src: string, lang?: string): string => withLang(lang, () => md.parse(src, { async: false }));
/** one line of markdown, no wrapping paragraph */
export const renderInline = (src: string, lang?: string): string => withLang(lang, () => md.parseInline(src, { async: false }) as string);

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

const KICKER = /^\*\*([^*\n]{1,60}?)\.\*\*\s*/;
const WHY_LABEL = /^\*\*Why now\s*[:.]?\*\*\s*:?\s*/i;
/** the bold lead-in of a paragraph ("**Mark it.** You mark…" -> "Mark it"), and the rest of the text */
const splitKicker = (text: string): { title: string; body: string } => {
  const m = KICKER.exec(text.trimStart());
  return m ? { title: m[1], body: text.trimStart().slice(m[0].length) } : { title: "", body: text };
};
/**
 * The intro is the first paragraph: the old "**Why now:**" one (label stripped) or, in new lessons,
 * a first paragraph without a bold kicker. Returns its rendered html and removes it from `idea`.
 */
function takeIntro(idea: Token[]): string {
  const wi = idea.findIndex((t) => t.type === "paragraph");
  if (wi < 0) return "";
  const raw = idea[wi].raw.trimStart();
  const labelled = WHY_LABEL.test(raw);
  if (!labelled && KICKER.test(raw)) return "";
  const text = (idea[wi] as Tokens.Paragraph).text.replace(WHY_LABEL, "");
  idea.splice(wi, 1);
  return `<p>${md.parseInline(text, { async: false }) as string}</p>`;
}

/** Split a lesson into why / task / first code / rest of the idea. null when there is no "## Your turn" list. */
export function splitLesson(src: string, lang?: string): LessonParts | null {
  return withLang(lang, () => splitLessonIn(src));
}
function splitLessonIn(src: string): LessonParts | null {
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
  const why = takeIntro(idea);
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

/** syntax-highlight `text` for the language hljs knows (plain escaped otherwise) */
export const highlight = (text: string, lang: string): string =>
  lang && hljs.getLanguage(lang) ? hljs.highlight(text, { language: lang }).value : esc(text);

export interface CardCode { raw: string; lang: string; html: string }
export interface Card {
  /** the bold kicker ("Mark it"), shown as the card's title line; "" when the paragraph has none */
  title: string;
  /** rendered point (paragraph or one bullet), plus any non-code block that follows it */
  html: string;
  /** code blocks that follow the point's text */
  codes: CardCode[];
}
export interface LessonCards { why: string; cards: Card[]; tasks: string[] }

/**
 * Split a lesson into the Learn stage: the "Why now" paragraph, one card per idea paragraph or bullet
 * (card N is point N, matching `quest.points[].after`), and the tasks. A code block rides on the card
 * whose text precedes it. null when there is no "## Your turn" list.
 */
export function lessonCards(src: string, lang?: string): LessonCards | null {
  return withLang(lang, () => lessonCardsIn(src));
}
function lessonCardsIn(src: string): LessonCards | null {
  const base = splitLessonIn(src);
  if (!base) return null;
  const toks = md.lexer(src);
  const ti = toks.findIndex((t) => t.type === "heading" && t.depth === 2 && /^your turn$/i.test(t.text.trim()));
  let end = toks.length;
  for (let i = ti + 1; i < toks.length; i++) {
    const t = toks[i];
    if (t.type === "heading" && t.depth <= 2) { end = i; break; }
  }
  const section = toks.slice(ti + 1, end);
  const list = section.find((t): t is Tokens.List => t.type === "list");
  const idea = [...toks.slice(0, ti), ...section.filter((t) => t !== list), ...toks.slice(end)].filter((t) => t.type !== "space");
  takeIntro(idea);

  const cards: Card[] = [];
  let lead = ""; // blocks that come before the first point
  let leadCodes: CardCode[] = [];
  const push = (html: string, title = "") => { cards.push({ title, html: lead + html, codes: leadCodes }); lead = ""; leadCodes = []; };
  const para = (text: string) => {
    const k = splitKicker(text);
    push(`<p>${md.parseInline(k.body, { async: false }) as string}</p>`, k.title);
  };
  const extra = (html: string) => {
    if (cards.length) cards[cards.length - 1].html += html;
    else lead += html;
  };
  for (const t of idea) {
    if (t.type === "paragraph") para(t.text);
    else if (t.type === "list") {
      for (const it of t.items) {
        para(it.text);
      }
    } else if (t.type === "code") {
      const l = (t.lang ?? "").split(/\s/)[0];
      const c = { raw: t.text, lang: l, html: highlight(t.text, l) };
      (cards.length ? cards[cards.length - 1].codes : leadCodes).push(c);
    } else extra(md.parser([t]));
  }
  if (!cards.length && (lead || leadCodes.length)) push("");
  return { why: base.why, cards, tasks: base.tasks };
}
