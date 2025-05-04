# `declarationOrderGroupBy`

Control how declarations are considered as a group when sorting.

Possible values:

- `"nonDeclaration"`: Non-declaration statements are considered as the boundaries of declaration group.
- `"nonDeclarationAndEmptyLine"`: Non-declaration statements and empty lines (without comments) are considered as the boundaries of declaration group.

Default value is `"nonDeclaration"`.

## Example for `"nonDeclaration"`

Supposed declarations are sorted by `"alphabetical"` strategy.

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0vJLFOo5lJQKMkvsFIwsAayclLTSqBMNYXqWiBVnplSkmGlYGhgoAoSzUjNTM8ogfFrAf%2FOR5hCAAAA&config=H4sIAAAAAAAAA6vmUlBQSklNzkksSizJzM%2FzL0pJLVKyUlBKzCnISExKLclMTsxR0sGmyr0ov7TAqRKkOC8%2FzwUhq8RVCwAiMIaVVwAAAA%3D%3D&syntax=css)

```css
div {
  left: 0;
  top: 0;
  & {}
  height: 100%;
  width: 100%;
}
```

## Example for `"nonDeclarationAndEmptyLine"`

Supposed declarations are sorted by `"alphabetical"` strategy.

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0vJLFOo5lJQKMkvsFIwsAayclLTSqBMNYXqWiBVnplSkmGlYGhgoAoSzUjNTM8ogfGBAgWJKSmZeelQTbmJRemZeWBOLQB7IyKuXgAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQSklNzkksSizJzM%2FzL0pJLVKyUlBKzCnISExKLclMTsxR0sGmyr0ov7TAqRKkOC8%2FzwUh65iX4ppbUFLpk5mXqsRVCwCbS62LYwAAAA%3D%3D&syntax=css)

```css
div {
  left: 0;
  top: 0;
  & {}
  height: 100%;
  width: 100%;

  margin: 0;
  padding: 0;
}
```
