# `blockSelectorLinebreak`

Control line break behavior after selector commas.

Possible options:

- `"always"`: Always insert line break after comma.
- `"consistent"`: If the whole selector can be put on a single line, there won't be line breaks; otherwise, there will be line breaks after each comma.
- `"wrap"`: Selector will be put on one line as possible. Once it exceeds [`printWidth`](./print-width.md), line break will be inserted where the code exceeds [`printWidth`](./print-width.md).

Default option is `"consistent"`.

## Example for `"always"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA8sw1FHIMAJiYyA2AWJTIDZTqK4FAFCmhucZAAAA&config=H4sIAAAAAAAAA6vmUlBQSsrJT84OTs1JTS7JL%2FLJzEtNKkpNzFayUlBKzClPrCxW4qoFAGwR2RIoAAAA&syntax=css)

```css
h1,
h2,
h3,
h4,
h5,
h6 {}
```

## Example for `"consistent"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA8sw1FHIMAJiYyA2AWJTIDZTqK7l0itLLarUxUnk5Oel6xan5qQml%2BQX6SgQUI1PM9AyAMHHwEmGAAAA&config=H4sIAAAAAAAAA6vmUlBQSsrJT84OTs1JTS7JL%2FLJzEtNKkpNzFayUlBKzs8rziwuSc0rUeKqBQCxE9k2LAAAAA%3D%3D&syntax=css)

In this example, the first selector can be put on one line,
but the second selector can't, so there will be line breaks.

```css
h1, h2, h3, h4, h5, h6 {}
.very-very-very-very-very-very-long-selector,
.very-very-very-very-very-very-very-very-very-long-selector {}
```

## Example for `"wrap"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAAz1QS27FMAjc%2BxQ9gDf93gcHktDYJs%2BfSFXVu3dIpS6G4DDMAPsoOYZk%2FBUD6xVDP6nGYOlTlhGDro2KxLA%2FAy%2FAK%2FAGvAMfMZzozbYcj2kDtLMhUEoNkblJ7zEs6pXFGJEFZrzCQArEy4ZQwTkSx%2FCAORUo9kIZvD6aVTD6TB5QuAjCeChEXAgDMhoN%2BQQy%2Fq8qmbugslqDR6bknlk2qaAOStmnoXOoYYzxt%2FlYzdAydiEnNU8B5NSGLt5CXfnurBd132SQ5u6G278a8ukXcDWBiMvd362Zz1%2Bkzhgq%2BZlx3runz1Ko%2BQzqh0Z%2BwGyyGvaFpT19%2F%2FwCsipC3KUBAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSsrJT84OTs1JTS7JL%2FLJzEtNKkpNzFayUlAqL0osUOKqBQC%2Bds6pJgAAAA%3D%3D&syntax=css)

```css
html, body, div, span, object, iframe, h1, h2, h3, h4, h5, h6, p, blockquote,
pre, abbr, address, cite, code, del, dfn, em, img, ins, kbd, q, samp, small,
strong, sub, sup, var, b, i, dl, dt, dd, ol, ul, li, fieldset, form, label,
legend, table, caption, tbody, tfoot, thead, tr, th, td, article, aside, canvas,
details, figcaption, figure, footer, header, hgroup, menu, nav, section,
summary, time, mark, audio, video {}
```

## Overriding or ignoring

Selector of specific block can be overridden or ignored by adding a comment directive above the selector.

For example, to ignore it:

```css
/* malva-selector-override ignore */
h1, h2,
h3, h4,
h5, h6 {}
```

Or, to override with different options:

```css
/* malva-selector-override always */
.container,
.btn {}
```

This selector will be formatted as `"always"` disregarding the configuration.

To customize the comment directive text, see the [`selectorOverrideCommentDirective`](./selector-override-comment-directive.md) option.
