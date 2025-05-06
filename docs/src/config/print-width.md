# `printWidth`

The line width limitation that Malva should *(but not must)* avoid exceeding. Malva will try its best to keep line width less than this value, but it may exceed for some cases, for example, a very very long single word.

Default value is `80`.

## Example for `80`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjLzyuxUijNKSlK1E3Oz0tJzStOTVEozk3MydFNTiwoVjDUM0rNVVByyyxKVAhOzCtW0lEoBlK6xalFmWnWXLUA21q5SEcAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKijKzCsJz0wpyVCyUrAw4KoFADCfR78WAAAA&syntax=css)

```css
a {
  font: ultra-condensed small-caps 1.2em "Fira Sans", sans-serif;
}
```

## Example for `40`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjLzyuxUijNKSlK1E3Oz0tJzStOTVEozk3MydFNTiwoVjDUM0rNVVByyyxKVAhOzCtW0lEoBlK6xalFmWnWXLUA21q5SEcAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKijKzCsJz0wpyVCyUjAx4KoFAIggkfUWAAAA&syntax=css)

```css
a {
  font:
    ultra-condensed small-caps 1.2em
    "Fira Sans",
    sans-serif;
}
```
