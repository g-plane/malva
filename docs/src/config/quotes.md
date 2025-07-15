# `quotes`

Control the quotes of strings.

Possible values:

- `"alwaysDouble"`: Always use double quotes. Double quotes in strings will be escaped.
- `"alwaysSingle"`: Always use single quotes. Single quotes in strings will be escaped.
- `"preferDouble"`: Use double quotes as possible. However if there're double quotes in strings, quotes will be kept as-is.
- `"preferSingle"`: Use single quotes as possible. However if there're single quotes in strings, quotes will be kept as-is.

Default value is `"alwaysDouble"`.

This global option can be overridden by different syntax nodes:

- `attrSelector.quotes`

## Example for `"alwaysDouble"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7OySkpNyy9KVajmUlBIzs8rSc0rsVJQV7dG4SoB%2BbUAorGD2SsAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKizNL0ktVrJSUErMKU%2BsLHbJL03KSVXiqgUAT525jh4AAAA%3D&syntax=css)

```css
::before {
  content: "";
  content: "\"";
}
```

## Example for `"alwaysSingle"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7OySkpNyy9KVajmUlBIzs8rSc0rsVJQUrJG4aoD%2BbUAaQjZ0CsAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKizNL0ktVrJSUErMKU%2BsLA7OzEvPSVXiqgUA9%2BJjtR4AAAA%3D&syntax=css)

```css
::before {
  content: '';
  content: '\'';
}
```

## Example for `"preferDouble"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7OySkpNyy9KVajmUlBIzs8rSc0rsVJQV7dG4SoB%2BbUAorGD2SsAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKizNL0ktVrJSUCooSk1LLXLJL03KSVXiqgUAKHfkNR4AAAA%3D&syntax=css)

```css
::before {
  content: "";
  content: '"';
}
```

## Example for `"preferSingle"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7OySkpNyy9KVajmUlBIzs8rSc0rsVJQUrJG4aoD%2BbUAaQjZ0CsAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKizNL0ktVrJSUCooSk1LLQrOzEvPSVXiqgUAkAg%2BDh4AAAA%3D&syntax=css)

```css
::before {
  content: '';
  content: "'";
}
```
