# `hexColorLength`

Control the hex color values in short-hand form or long-hand form.

Possible values:

- `null`: Hex color values will be kept as-is.
- `"short"`: Hex color values will be converted to short-hand form.
- `"long"`: Hex color values will be converted to long-hand form.

Default value is `null`.

## Example for `null`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjOz8kvslJQTktLs0blgkVqAf9LeqomAAAA&config=H4sIAAAAAAAAA6vmqgUAqLu%2BcwMAAAA%3D&syntax=css)

```css
a {
  color: #fff;
  color: #ffffff;
}
```

## Example for `"short"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjOz8kvslJQTgMDa65aADLG74sXAAAA&config=H4sIAAAAAAAAA6vmUlBQykitcM7PyS%2FySc1LL8lQslJQKs7ILypR4qoFAAuXqYIfAAAA&syntax=css)

```css
a {
  color: #fff;
}
```

## Example for `"long"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjOz8kvslJQTktLs%2BaqBQABymepFAAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQykitcM7PyS%2FySc1LL8lQslJQysnPS1fiqgUAuPAwgx4AAAA%3D&syntax=css)

```css
a {
  color: #ffffff;
}
```
