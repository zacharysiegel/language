# Visual design

The visual identity follows Eafora's: sharp, white paper and red ink, square corners, 1px borders, generous whitespace, no shadows, no animation. Eafora's `docs/design/README.md` is the long form; this page records how it applies to a course.

## Rules

- Colors are white, black, red, blue and 1px-rule grey, and nothing else. Red marks the one active thing: the current page, the current tab, the control under the pointer, a wrong answer. Blue is for links. Greys are rules and disabled states, never fills.
- Corners are square (1px on inputs and buttons). Borders are 1px. A sheet is a bordered white `.panel`; it has no shadow.
- Type is the structure. A section is a line of capitalised type over a rule; there are no header bars. Numerals are tabular (`.numeric`, IBM Plex Mono). Body text is 14px with generous leading for running explanation.
- Correct and incorrect are shown by border and mark, not by fill: an ink border and a check for correct, a red border and a cross for incorrect.
- A tag (`.tag`) is a bordered uppercase word carrying a category, such as a part of speech or a status. It never carries a number or a sentence.
- Icons are single-stroke, 1.25px, no fill, in `currentColor`.
- Focus looks exactly like hover.
- Progress is a thin red bar over a hairline. There are no spinners.
- A control whose request is still on its way shows a static ellipsis in place of its glyph, in red, once the pending indication delay has passed; a loading page shows its status line after the same delay.

## Where it lives

`web/style/` mirrors Eafora's layout: `_tokens.scss` holds every color, space and type size as a custom property, and nothing else uses a raw one (layout dimensions local to one component may be literals); `_reset.scss`, `_typography.scss` and `_layout.scss` set the page; `components/` holds the reusable parts (button, panel, tag, field, progress, table); the feature partials (`_chrome`, `_home`, `_lesson`, `_practice`, `_review`) hold what belongs to one screen. Plain Sass, no utility-class library.
