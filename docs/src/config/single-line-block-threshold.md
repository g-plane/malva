# `singleLineBlockThreshold`

Control the threshold value for putting block on a single line.
If the number of statements in a block is less than or equal to this value, the block will be put on a single line as possible,
but when the code can't fit on single line, it will still break into multiple lines.

This is especially useful for increasing readability when writing atomic CSS. For example:

```css
.border-0 { border-width: 0px; }
.border-1 { border-width: 1px; }
.border-2 { border-width: 2px; }
.border-3 { border-width: 3px; }
.border-4 { border-width: 4px; }
.border-5 { border-width: 5px; }
```

Default value value is `null` which means always break into multiple lines. The option value can be an integer which is greater than or equal to 0.

## Example for `null`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUNDXUkjOz81NzStR0NLnquXiSgQLl2emlGRYKRhYc9UCAMWBkeUoAAAA&config=H4sIAAAAAAAAA6vmUlBQKs7MS89J9cnMS3XKyU%2FODskoSi3OyM9JUbJSyCvNyeGqBQBUWHQkJgAAAA%3D%3D&syntax=css)

```css
a {
  /* comment */
}

a {
  width: 0;
}
```

## Example for `1`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUNDXUkjOz81NzStR0NLnquXiSgQLl2emlGRYKRhYYxNSUMhIzUzPKIHIAwD%2FjJ20SAAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQKs7MS89J9cnMS3XKyU%2FODskoSi3OyM9JUbJSMOSqBQD%2F4H9jIwAAAA%3D%3D&syntax=css)

```css
a { /* comment */ }

a { width: 0; }

a {
  width: 0;
  height: 0;
}
```

## Example for `2`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUNDXUkjOz81NzStR0NLnquXiSgQLl2emlGRYKRhYYxNSUMhIzUzPKIHIAwD%2FjJ20SAAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQKs7MS89J9cnMS3XKyU%2FODskoSi3OyM9JUbJSMOKqBQCmXjlhIwAAAA%3D%3D&syntax=css)

```css
a { /* comment */ }

a { width: 0; }

a { width: 0; height: 0; }
```
