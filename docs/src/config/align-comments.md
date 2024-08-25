# `alignComments`

Control whether to tweak multi-line comments indentation.

Default option value is `true`.

## Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSAAJ9LYXk%2FNzc1LwShZzMvFQFQ7AoEKCIGmEVNQaLaumDqZLUihLdlNTk%2FKLEksz8PCuFvPy8VGuuWgAzUTX%2BaQAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQSszJTM9zzs%2FNTc0rKVayUkhLzClO5aoFAIntfVEcAAAA&syntax=css)

When formatting the 4-space indented CSS into 2-space indented CSS:

```css
a {
    /* comment line 1
       comment line 2
       comment line 3
    */
    text-decoration: none;
}
```

will be formatted as:

```css
a {
  /* comment line 1
       comment line 2
       comment line 3
    */
  text-decoration: none;
}
```

## Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSAAJ9LYXk%2FNzc1LwShZzMvFQFQ7AoEKCIGmEVNQaLaumDqZLUihLdlNTk%2FKLEksz8PCuFvPy8VGuuWgAzUTX%2BaQAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQSszJTM9zzs%2FNTc0rKVayUigpKk3lqgUAiNln3BsAAAA%3D&syntax=css)

When formatting the 4-space indented CSS into 2-space indented CSS:

```css
a {
    /* comment line 1
       comment line 2
       comment line 3
    */
    text-decoration: none;
}
```

will be formatted as:

```css
a {
  /* comment line 1
     comment line 2
     comment line 3
  */
  text-decoration: none;
}
```
