# `attrValueQuotes`

Control whether should add quotes to attribute value in selector or not if it's not quoted.

Possible values:

- `"always"`: Always add quotes.
- `"ignore"`: Don't add quotes to those that're not quoted. For quoted value, quotes won't be removed.

Default value is `"always"`.

## Example for `"always"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA4vOTq20LUvMKU2NVaiu5QIAJVipng8AAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSiwpKQpLzClNDSzNL0ktVrICCuWUJ1YWK3HVAgAOImZZIQAAAA%3D%3D&syntax=css)

```css
[key="value"] {}
```

## Example for `"ignore"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA4vOTq20LUvMKU2NVaiu5QIAJVipng8AAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSiwpKQpLzClNDSzNL0ktVrJSUMpMz8svSlXiqgUAfx06dSEAAAA%3D&syntax=css)

```css
[key=value] {}
```

## See also

- [`quotes`](./quotes.md) option for controlling using single or double quotes.
