# `operatorLinebreak`

Control whether line break should come before or after operators.

Possible values:

- `"before"`: Line break will come before operators.
- `"after"`: Line break will come after operators.

Default value is `"after"`.

## Example for `"before"`

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

## Example for `"after"`

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
