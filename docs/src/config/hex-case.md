# `hexCase`

Control the case of hex color values.

Possible options:

- `"lower"`: Hex color values will be converted to lower case.
- `"upper"`: Hex color values will be converted to upper case.
- `"ignore"`: Hex color values will be kept as-is.

Default option is `"lower"`.

## Example for `"lower"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEhKTM5OL8ovzUuxUlB2S3Oz5qoFAAjbXCsZAAAA&config=H4sIAAAAAAAAA6vmUlBQykitcE4sTlWyUlDKyS9PLVLiqgUAAAWeeBgAAAA%3D&syntax=css)

```css
a {
  background: #fff;
}
```

## Example for `"upper"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEhKTM5OL8ovzUuxUlB2S3Oz5qoFAAjbXCsZAAAA&config=H4sIAAAAAAAAA6vmUlBQykitcE4sTlWyUlAqLShILVLiqgUAn%2FP5lxgAAAA%3D&syntax=css)

```css
a {
  background: #FFF;
}
```

## Example for `"ignore"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEhKTM5OL8ovzUuxUlB2S3Oz5qoFAAjbXCsZAAAA&config=H4sIAAAAAAAAA6vmUlBQykitcE4sTlWyUlDKTM%2FLL0pV4qoFAGVAgmYZAAAA&syntax=css)

```css
a {
  background: #FfF;
}
```
