# `keyframeSelectorNotation`

Control whether to use percentage or keyword (`from` and `to`) notation as keyframe selectors.

Possible values:

- `null`: Keyframe selector notation will be kept as-is.
- `"keyword"`: Use keyword notation. This only affects `0%` and `100%`. For other percentage values, they will be kept as-is.
- `"percentage"`: Use percentage notation.

Default value is `null`.

## Example for `null`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA3PITq1MK0rMTS1WSMvPV6jmUlBIK8rPVaiuBbJMDVQhDEMDCKsWABlFCDAvAAAA&config=H4sIAAAAAAAAA6uuBQBDv6ajAgAAAA%3D%3D&syntax=css)

```css
@keyframes foo {
  from {}
  50% {}
  100% {}
}
```

## Example for `"keyword"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA3PITq1MK0rMTS1WSMvPV6jmUlBIK8rPVaiuBbJMDVQhDEMDCKsWABlFCDAvAAAA&config=H4sIAAAAAAAAA6vmUlBQyk6tTCtKzE0NTs1JTS7JL%2FLLL0ksyczPU7ICy5XnF6UocdUCACuoVucrAAAA&syntax=css)

```css
@keyframes foo {
  from {}
  50% {}
  to {}
}
```

## Example for `"percentage"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA3PITq1MK0rMTS1WSMvPV6jmUlBIK8rPVaiuBbJMDVQhDEMDCKsWABlFCDAvAAAA&config=H4sIAAAAAAAAA6vmUlBQyk6tTCtKzE0NTs1JTS7JL%2FLLL0ksyczPU7JSUCpILUpOzStJTE9V4qoFAIT8kDouAAAA&syntax=css)

```css
@keyframes foo {
  0% {}
  50% {}
  100% {}
}
```
