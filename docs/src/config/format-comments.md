# `formatComments`

Control whether whitespace should be inserted at the beginning and end of comments.

Though this option is set to `false`, comments contain leading or trailing whitespace will still be kept as-is.

Default value is `false`.

> This option is renamed from `padComments` which is deprecated.

## Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA9PXSs7PzU3NKynW0ufS11KA8RS09AGs222%2FGwAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQSssvyk0scc7PzU3NKylWslJIS8wpTuWqBQCk3Eq9HQAAAA%3D%3D&syntax=css)

```css
/*comments*/
/* comments */
```

## Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA9PXSs7PzU3NKynW0ufS11KA8RS09AGs222%2FGwAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQSssvyk0scc7PzU3NKylWslIoKSpN5aoFAEnSpIgcAAAA&syntax=css)

```css
/* comments */
/* comments */
```
