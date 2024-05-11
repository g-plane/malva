# Configuration

Options name in this page are in camel case.
If you're using Malva as a Rust crate, please use snake case instead.

## `printWidth`

The line width limitation that Malva should *(but not must)* avoid exceeding. Malva will try its best to keep line width less than this value, but it may exceed for some cases, for example, a very very long single word.

Default option is `80`.

### Example for `80`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjLzyuxUijNKSlK1E3Oz0tJzStOTVEozk3MydFNTiwoVjDUM0rNVVByyyxKVAhOzCtW0lEoBlK6xalFmWnWXLUA21q5SEcAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKijKzCsJz0wpyVCyUrAw4KoFADCfR78WAAAA&syntax=css)

```css
a {
  font: ultra-condensed small-caps 1.2em "Fira Sans", sans-serif;
}
```

### Example for `40`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjLzyuxUijNKSlK1E3Oz0tJzStOTVEozk3MydFNTiwoVjDUM0rNVVByyyxKVAhOzCtW0lEoBlK6xalFmWnWXLUA21q5SEcAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKijKzCsJz0wpyVCyUjAx4KoFAIggkfUWAAAA&syntax=css)

```css
a {
  font:
    ultra-condensed small-caps 1.2em
    "Fira Sans",
    sans-serif;
}
```

## `useTabs`

Specify use space or tab for indentation.

Default option is `false`.

### Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUChJrSjRTUlNzi9KLMnMz7NSyMvPS7XmqgUAPo3pNh4AAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKi1ODUlMKlayUkhLzClO5aoFAC9EjqIWAAAA&syntax=css)

### Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUChJrSjRTUlNzi9KLMnMz7NSyMvPS7XmqgUAPo3pNh4AAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKi1ODUlMKlayUigpKk3lqgUAmje%2FlhUAAAA%3D&syntax=css)

## `indentWidth`

Size of indentation. When enabled `useTabs`, this option may be disregarded,
since only one tab will be inserted when indented once.

Default option is `2`. This can't be zero.

Examples below will are based on `useTabs: false`.

### Example for `2`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUChJrSjRTUlNzi9KLMnMz7NSyMvPS7XmqgUAPo3pNh4AAAA%3D&config=H4sIAAAAAAAAA6vmUlBQysxLSc0rCc9MKclQslIw4qoFAJDs1WEWAAAA&syntax=css)

```css
a {
  text-decoration: none;
}
```

### Example for `4`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUChJrSjRTUlNzi9KLMnMz7NSyMvPS7XmqgUAPo3pNh4AAAA%3D&config=H4sIAAAAAAAAA6vmUlBQysxLSc0rCc9MKclQslIw4aoFACKQWGUWAAAA&syntax=css)

```css
a {
    text-decoration: none;
}
```

## `lineBreak`

Specify use `\n` (LF) or `\r\n` (CRLF) for line break.

Default option is `"lf"`. Possible options are `"lf"` and `"crlf"`.

### Example for `"lf"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUChJrSjRTUlNzi9KLMnMz7NSyMvPS7XmqgUAPo3pNh4AAAA%3D&config=H4sIAAAAAAAAA6vmUlBQysnMS3UqSk3MVrICctKUuGoBE6%2BwKRcAAAA%3D&syntax=css)

### Example for `"crlf"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUChJrSjRTUlNzi9KLMnMz7NSyMvPS7XmqgUAPo3pNh4AAAA%3D&config=H4sIAAAAAAAAA6vmUlBQysnMS3UqSk3MVrJSUEouyklT4qoFANXW2C0ZAAAA&syntax=css)

## `hexCase`

Control the case of hex color values.

Possible options:

- `"lower"`: Hex color values will be converted to lower case.
- `"upper"`: Hex color values will be converted to upper case.
- `"ignore"`: Hex color values will be kept as-is.

Default option is `"lower"`.

### Example for `"lower"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEhKTM5OL8ovzUuxUlB2S3Oz5qoFAAjbXCsZAAAA&config=H4sIAAAAAAAAA6vmUlBQykitcE4sTlWyUlDKyS9PLVLiqgUAAAWeeBgAAAA%3D&syntax=css)

```css
a {
  background: #fff;
}
```

### Example for `"upper"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEhKTM5OL8ovzUuxUlB2S3Oz5qoFAAjbXCsZAAAA&config=H4sIAAAAAAAAA6vmUlBQykitcE4sTlWyUlAqLShILVLiqgUAn%2FP5lxgAAAA%3D&syntax=css)

```css
a {
  background: #FFF;
}
```

### Example for `"ignore"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEhKTM5OL8ovzUuxUlB2S3Oz5qoFAAjbXCsZAAAA&config=H4sIAAAAAAAAA6vmUlBQykitcE4sTlWyUlDKTM%2FLL0pV4qoFAGVAgmYZAAAA&syntax=css)

```css
a {
  background: #FfF;
}
```

## `hexColorLength`

Control the hex color values in short-hand form or long-hand form.

Possible options:

- `null`: Hex color values will be kept as-is.
- `"short"`: Hex color values will be converted to short-hand form.
- `"long"`: Hex color values will be converted to long-hand form.

Default option is `null`.

### Example for `null`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjOz8kvslJQTktLs0blgkVqAf9LeqomAAAA&config=H4sIAAAAAAAAA6vmqgUAqLu%2BcwMAAAA%3D&syntax=css)

```css
a {
  color: #fff;
  color: #ffffff;
}
```

### Example for `"short"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjOz8kvslJQTgMDa65aADLG74sXAAAA&config=H4sIAAAAAAAAA6vmUlBQykitcM7PyS%2FySc1LL8lQslJQKs7ILypR4qoFAAuXqYIfAAAA&syntax=css)

```css
a {
  color: #fff;
}
```

### Example for `"long"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjOz8kvslJQTktLs%2BaqBQABymepFAAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQykitcM7PyS%2FySc1LL8lQslJQysnPS1fiqgUAuPAwgx4AAAA%3D&syntax=css)

```css
a {
  color: #ffffff;
}
```

## `quotes`

Control the quotes of strings.

Possible options:

- `"alwaysDouble"`: Always use double quotes. Double quotes in strings will be escaped.
- `"alwaysSingle"`: Always use single quotes. Single quotes in strings will be escaped.
- `"preferDouble"`: Use double quotes as possible. However if there're double quotes in strings, quotes will be kept as-is.
- `"preferSingle"`: Use single quotes as possible. However if there're single quotes in strings, quotes will be kept as-is.

Default option is `"alwaysDouble"`.

### Example for `"alwaysDouble"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7OySkpNyy9KVajmUlBIzs8rSc0rsVJQV7dG4SoB%2BbUAorGD2SsAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKizNL0ktVrJSUErMKU%2BsLHbJL03KSVXiqgUAT525jh4AAAA%3D&syntax=css)

```css
::before {
  content: "";
  content: "\"";
}
```

### Example for `"alwaysSingle"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7OySkpNyy9KVajmUlBIzs8rSc0rsVJQUrJG4aoD%2BbUAaQjZ0CsAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKizNL0ktVrJSUErMKU%2BsLA7OzEvPSVXiqgUA9%2BJjtR4AAAA%3D&syntax=css)

```css
::before {
  content: '';
  content: '\'';
}
```

### Example for `"preferDouble"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7OySkpNyy9KVajmUlBIzs8rSc0rsVJQV7dG4SoB%2BbUAorGD2SsAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKizNL0ktVrJSUCooSk1LLXLJL03KSVXiqgUAKHfkNR4AAAA%3D&syntax=css)

```css
::before {
  content: "";
  content: '"';
}
```

### Example for `"preferSingle"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7OySkpNyy9KVajmUlBIzs8rSc0rsVJQUrJG4aoD%2BbUAaQjZ0CsAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKizNL0ktVrJSUCooSk1LLQrOzEvPSVXiqgUAkAg%2BDh4AAAA%3D&syntax=css)

```css
::before {
  content: '';
  content: "'";
}
```

## `operatorLinebreak`

Control whether line break should come before or after operators.

Possible options:

- `"before"`: Line break will come before operators.
- `"after"`: Line break will come after operators.

Default option is `"after"`.

### Example for `"before"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA22NMQqAMAxFd0%2BRRWiRgK56mthIDdRWUlFQvLugg4Ou%2F73HJzgKgE14GVtwFJxp6roEBLOSGsQsPPSkeBsWKnjmmZglehyTyp7iQuFlfVIePsVE6iX%2BBtlpCuF9sbYrzgu7xPo3mQAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQyi9ILUosyS%2FyycxLTSpKTcxWslJQSkpNyy9KVeKqBQAUkWQCIwAAAA%3D%3D&syntax=css)

```css
a {
  width: calc(
    100%
      - (
      var(--sidebar-width) + var(--padding-horizontal) + var(--border-width)
        + var(--margin-horizontal)
        + var(--scrollbar-width)
    )
  );
}
```

### Example for `"after"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA22NMQqAMAxFd0%2BRRWiRgK56mthIDdRWUlFQvLugg4Ou%2F73HJzgKgE14GVtwFJxp6roEBLOSGsQsPPSkeBsWKnjmmZglehyTyp7iQuFlfVIePsVE6iX%2BBtlpCuF9sbYrzgu7xPo3mQAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQyi9ILUosyS%2FyycxLTSpKTcxWslJQSkwrSS1S4qoFAKcZSFsiAAAA&syntax=css)

```css
a {
  width: calc(
    100% -
      (
      var(--sidebar-width) + var(--padding-horizontal) + var(--border-width) +
        var(--margin-horizontal) +
        var(--scrollbar-width)
    )
  );
}
```

## `blockSelectorLinebreak`

Control line break behavior after selector commas.

Possible options:

- `"always"`: Always insert line break after comma.
- `"consistent"`: If the whole selector can be put on a single line, there won't be line breaks; otherwise, there will be line breaks after each comma.
- `"wrap"`: Selector will be put on one line as possible. Once it exceeds `printWidth`, line break will be inserted where the code exceeds `printWidth`.

Default option is `"consistent"`.

### Example for `"always"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA8sw1FHIMAJiYyA2AWJTIDZTqK4FAFCmhucZAAAA&config=H4sIAAAAAAAAA6vmUlBQSsrJT84OTs1JTS7JL%2FLJzEtNKkpNzFayUlBKzClPrCxW4qoFAGwR2RIoAAAA&syntax=css)

```css
h1,
h2,
h3,
h4,
h5,
h6 {}
```

### Example for `"consistent"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA8sw1FHIMAJiYyA2AWJTIDZTqK7l0itLLarUxUnk5Oel6xan5qQml%2BQX6SgQUI1PM9AyAMHHwEmGAAAA&config=H4sIAAAAAAAAA6vmUlBQSsrJT84OTs1JTS7JL%2FLJzEtNKkpNzFayUlBKzs8rziwuSc0rUeKqBQCxE9k2LAAAAA%3D%3D&syntax=css)

In this example, the first selector can be put on one line,
but the second selector can't, so there will be line breaks.

```css
h1, h2, h3, h4, h5, h6 {}
.very-very-very-very-very-very-long-selector,
.very-very-very-very-very-very-very-very-very-long-selector {}
```

### Example for `"wrap"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAAz1QS27FMAjc%2BxQ9gDf93gcHktDYJs%2BfSFXVu3dIpS6G4DDMAPsoOYZk%2FBUD6xVDP6nGYOlTlhGDro2KxLA%2FAy%2FAK%2FAGvAMfMZzozbYcj2kDtLMhUEoNkblJ7zEs6pXFGJEFZrzCQArEy4ZQwTkSx%2FCAORUo9kIZvD6aVTD6TB5QuAjCeChEXAgDMhoN%2BQQy%2Fq8qmbugslqDR6bknlk2qaAOStmnoXOoYYzxt%2FlYzdAydiEnNU8B5NSGLt5CXfnurBd132SQ5u6G278a8ukXcDWBiMvd362Zz1%2Bkzhgq%2BZlx3runz1Ko%2BQzqh0Z%2BwGyyGvaFpT19%2F%2FwCsipC3KUBAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSsrJT84OTs1JTS7JL%2FLJzEtNKkpNzFayUlAqL0osUOKqBQC%2Bds6pJgAAAA%3D%3D&syntax=css)

```css
html, body, div, span, object, iframe, h1, h2, h3, h4, h5, h6, p, blockquote,
pre, abbr, address, cite, code, del, dfn, em, img, ins, kbd, q, samp, small,
strong, sub, sup, var, b, i, dl, dt, dd, ol, ul, li, fieldset, form, label,
legend, table, caption, tbody, tfoot, thead, tr, th, td, article, aside, canvas,
details, figcaption, figure, footer, header, hgroup, menu, nav, section,
summary, time, mark, audio, video {}
```

## `omitNumberLeadingZero`

Control whether omit leading zero before dot of numbers or not.

Default option is `false`.

### Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUCjPTCnJsFIw0DMsqLDmqgUA5MKZYRUAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQys%2FNLPErzU1KLfJJTUzJzEuPSi3KV7JSSEvMKU7lqgUAj86rjiQAAAA%3D&syntax=css)

```css
a {
  width: 0.1px;
}
```

### Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUCjPTCnJsFIw0DMsqLDmqgUA5MKZYRUAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQys%2FNLPErzU1KLfJJTUzJzEuPSi3KV7JSKCkqTeWqBQAkHKtqIwAAAA%3D%3D&syntax=css)

```css
a {
  width: .1px;
}
```

## `trailingComma`

Control whether trailing comma should be inserted or not.

This only affects Sass list, Sass map, Sass parameters/arguments list,
Less list and Less parameters/arguments list.
CSS functions won't respect this option.

Default option is `false`.

### Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA1NJzs9Ly0y3UtDgUlAoyUjNTS2GsBUUcjOLS2BsBYWM1MSU1CIrBeWU5LTEZAMdqDBQe0lqHlCdsoGBpZlFEkw8LT%2B%2FBKzcwjTZ3DIZIqwJoVSKC4oy89KxmJ1mAjTbHNPsZCMTU5NUDLPT0pIMTS2QzAYSmtYAa6vVzdMAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKilKzMzJzEt3zs%2FNTVSyUkhLzClO5aoFAKpkbbAcAAAA&syntax=scss)

```scss
$config: (
  themes: (
    mist: (header: #dcfac0, content: #00968b, footer: #85c79c),
    $spring: (header: #f4fac7, content: #c2454e, footer: #ffb158)
  )
);
```

### Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA22MMQ6EMAwEe15hCQqQrginBAK8BowNKUhOJP8XgUDFFZZWs%2Bsp0Fk2Sw9lBhBW2sinDLAZH54MsNI4095DPiOPKD43ju%2BBbNzlQnSNnh7OzoVrrhW2HV60SmXhf7uxyx81y6hu32r8SiXppWaeaqWTOjuvGg5FOmEBzwAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQKilKzMzJzEt3zs%2FNTVSyUigpKk3lqgUAw%2BJ2whsAAAA%3D&syntax=scss)

```scss
$config: (
  themes: (
    mist: (header: #dcfac0, content: #00968b, footer: #85c79c),
    $spring: (header: #f4fac7, content: #c2454e, footer: #ffb158),
  ),
);
```

## `padComments`

Control whether whitespace should be inserted at the beginning and end of comments.

Though this option is set to `false`, comments contain leading or trailing whitespace will still be kept as-is.

Default option is `false`.

### Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA9PXSs7PzU3NKynW0ufS11KA8RS09AGs222%2FGwAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQKkhMcc7PzU3NKylWslJIS8wpTuWqBQCFZ6SlGgAAAA%3D%3D&syntax=css)

```css
/*comments*/
/* comments */
```

### Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA9PXSs7PzU3NKynW0ufS11KA8RS09AGs222%2FGwAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQKkhMcc7PzU3NKylWslIoKSpN5aoFAJ6EWNEZAAAA&syntax=css)

```css
/* comments */
/* comments */
```

## `linebreakInPseudoParens`

Control whether line break should be inserted in pseudo class/element parens
or not if current line is too long.

Default option is `false`.

### Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7Mqz0gtStVQLjCwSs5ITc5OTVGoU1AuyEmsROYnG8J4OkBJQ7xKjZCVGuFV6g%2FjaSpU1wIAa839xooAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQysnMS00qSk3M9swLKE4tTckPSCxKzStWslJIS8wpTuWqBQAAWvxOJgAAAA%3D%3D&syntax=css)

```css
:where(#p0:checked ~ #play:checked ~ #c1:checked, #p1:checked
  ~ #play:checked
  ~ #c2:checked, #p2:checked ~ #play:checked ~ #cO:checked) {}
```

### Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7Mqz0gtStVQLjCwSs5ITc5OTVGoU1AuyEmsROYnG8J4OkBJQ7xKjZCVGuFV6g%2FjaSpU1wIAa839xooAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQysnMS00qSk3M9swLKE4tTckPSCxKzStWslIoKSpN5aoFAFs%2BU4clAAAA&syntax=css)

```css
:where(
  #p0:checked ~ #play:checked ~ #c1:checked,
    #p1:checked ~ #play:checked ~ #c2:checked,
    #p2:checked ~ #play:checked ~ #cO:checked
) {}
```

## `declarationOrder`

Control the strategy of sorting CSS declarations (a.k.a. properties). If it's `null`, it won't sort CSS declarations.

Here're the strategies:

- `alphabetical`: Order in a simple alphabetical manner from a - z. This strategy will also sort unknown properties.
- `smacss`: Order from most important, flow affecting properties, to least important properties. Unknown properties won't be sorted.
- `concentric`: Order properties applying outside the box model, moving inward to intrinsic changes. Unknown properties won't be sorted.

For more detail, please read [https://github.com/Siilwyn/css-declaration-sorter](https://github.com/Siilwyn/css-declaration-sorter).

Default option value is `null`.

### Notes

- For all strategies, custom properties (whose name starts with `--`) won't be sorted.
- It will only sort adjacent CSS declarations. For example:

```css
div {
  width: 0;
  height: 0;
  span {}
  min-width: 0;
  min-height: 0;
}
```

Those declarations above the `span {}` and those declarations below the `span {}` will be sorted separately.

### Example for `"alphabetical"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0vJLFOo5lJQKM9MKcmwUjCwBrIzUjPTM0qgnJTM4oKcxEorhbSc1AprrloAWvsmnjEAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSklNzkksSizJzM%2FzL0pJLVKyUlBKzCnISExKLclMTsxR4qoFAOBP0aAoAAAA&syntax=scss)

```css
div {
  display: flex;
  height: 0;
  width: 0;
}
```

### Example for `"smacss"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0vJLFOo5lJQKM9MKcmwUjCwBrIzUjPTM0qgnJTM4oKcxEorhbSc1AprrloAWvsmnjEAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSklNzkksSizJzM%2FzL0pJLVKyUlAqzk1MLi5W4qoFAGowk7AiAAAA&syntax=scss)

```css
div {
  display: flex;
  width: 0;
  height: 0;
}
```

### Example for `"concentric"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0vJLFOo5lJQKM9MKcmwUjCwBrIzUjPTM0qgnJTM4oKcxEorhbSc1AprrloAWvsmnjEAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSklNzkksSizJzM%2FzL0pJLVKyUlBKzs9LTs0rKcpMVuKqBQA32QzHJgAAAA%3D%3D&syntax=scss)

```css
div {
  display: flex;
  width: 0;
  height: 0;
}
```

## `singleLineBlockThreshold`

Control the threshold value for putting block on a single line.
If the number of statements in a block is less than or equal to this value, the block will be put on a single line as possible,
but when the code can't fit on single line, it will still break into multiple lines.

This is especially useful for increasing readability when writing atomic CSS. For example:

```css
.border-0 { border-width: 0px; }
.border-1 { border-width: 1px; }
.border-2 { border-width: 2px; }
.border-3 { border-width: 3px; }
.border-4 { border-width: 4px; }
.border-5 { border-width: 5px; }
```

Default option value is `null` which means always break into multiple lines. The option value can be an integer which is greater than or equal to 0.

### Example for `null`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUNDXUkjOz81NzStR0NLnquXiSgQLl2emlGRYKRhYc9UCAMWBkeUoAAAA&config=H4sIAAAAAAAAA6vmUlBQKs7MS89J9cnMS3XKyU%2FODskoSi3OyM9JUbJSyCvNyeGqBQBUWHQkJgAAAA%3D%3D&syntax=css)

```css
a {
  /* comment */
}

a {
  width: 0;
}
```

### Example for `1`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUNDXUkjOz81NzStR0NLnquXiSgQLl2emlGRYKRhYYxNSUMhIzUzPKIHIAwD%2FjJ20SAAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQKs7MS89J9cnMS3XKyU%2FODskoSi3OyM9JUbJSMOSqBQD%2F4H9jIwAAAA%3D%3D&syntax=css)

```css
a { /* comment */ }

a { width: 0; }

a {
  width: 0;
  height: 0;
}
```

### Example for `2`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUNDXUkjOz81NzStR0NLnquXiSgQLl2emlGRYKRhYYxNSUMhIzUzPKIHIAwD%2FjJ20SAAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQKs7MS89J9cnMS3XKyU%2FODskoSi3OyM9JUbJSMOKqBQCmXjlhIwAAAA%3D%3D&syntax=css)

```css
a { /* comment */ }

a { width: 0; }

a { width: 0; height: 0; }
```
