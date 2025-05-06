# `singleLineTopLevelDeclarations`

Control whether to force to format all top-level declarations on a single line.

When this option is `true`, trailing semicolons are removed.

Most of the time, you don't need to set this option, because declarations at top level are invalid,
but if you're formatting HTML's `style` attribute, you may want to set this to `true`.

Default value is `false`.

## Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAAyvPTCnJsFIwNDAoqLDmykjNTM8ogXEBfacJkBwAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKs7MS89J9cnMSw3JL%2FBJLUvNcUlNzkksSizJzM8rVrJSSEvMKU7lqgUASka%2FoS0AAAA%3D&syntax=css)

```css
width: 100px;
height: 100px;
```

## Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAAyvPTCnJsFIwNDAoqLDmykjNTM8ogXEBfacJkBwAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKs7MS89J9cnMSw3JL%2FBJLUvNcUlNzkksSizJzM8rVrJSKCkqTeWqBQAYXl8RLAAAAA%3D%3D&syntax=css)

```css
width: 100px; height: 100px
```
