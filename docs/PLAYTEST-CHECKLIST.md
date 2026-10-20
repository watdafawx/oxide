# Blind playtest checklist

The question for every quest: **could a learner who knows nothing beyond the earlier quests of this
track solve it from the lesson, the quest labels, the starter and its tests?** Explain → do → reward.
Something that first appears in the exercise or the quiz is a content bug (see `docs/CONTENT.md`,
"Explain, then ask").

## Per quest

1. Copy the quest (games flavour when it exists, plain otherwise) to `%TEMP%\oxide-playtest\<id>`.
   Never open `solution/` before step 4.
2. Read only `lesson.md`, the objective labels in `quest.toml` (hints hidden), the starter and its tests.
3. Solve it. Rust: `cargo test` with `CARGO_TARGET_DIR=%TEMP%\oxide-playtest\target`.
   Lua: copy to the workspace and use content-check, or the embedded runner. Java: `gradlew.bat test` on JDK 21.
   Open hints one at a time only when stuck, and record which.
4. Then open `solution/` and audit: every call, keyword, macro, pattern and import it uses must be
   shown in this lesson or an earlier quest of the same track. List each one that is not.
5. Read quests: answer the quiz from the lesson text alone. Every answer must be decided by a sentence
   in that lesson. Right answer must not be the longest choice.
6. Check both flavours when the quest has them: the games lesson must teach what the games solution
   uses, the plain lesson what the plain solution uses.

## Fix, then record

- Fix in place: add the missing line/example to the lesson (keep it one screen; bold the term), make
  the `// TODO:` say what to write, make hint N name the thing, fix quiz choices. Never put the solution
  in the lesson.
- Lesson must stay skimmable: Why now (1-2 sentences), idea in 3-6 short paragraphs or bullets, one code
  block, `## Your turn` with 2-4 bullets naming the file and function.
- Minutes honest for a beginner; bump when the quest needs more than planned.
- `cargo run -p content-check -- <world>` green for every touched world before committing.
- One commit per world, explicit paths only (`git add content/tracks/<track>/worlds/<world>`), message
  `<Track> world NN: <what changed>`. Do not touch worlds you do not own.
- Report: `docs/PLAYTEST-<date>-<worlds>.md` with a per-quest table (quest, solved, hints used,
  verdict ok/unclear/broken, what was missing, what was fixed). Keep it honest: say when you knew the
  answer from experience rather than from the lesson.
