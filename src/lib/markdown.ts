import { Marked } from "marked";
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
