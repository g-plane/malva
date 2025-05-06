# `declarationOrder`

Control the strategy of sorting CSS declarations (a.k.a. properties). If it's `null`, it won't sort CSS declarations.

Here're the available strategies:

- `alphabetical`: Order in a simple alphabetical manner from a - z. This strategy will also sort unknown properties.
- `smacss`: Order from most important, flow affecting properties, to least important properties. Unknown properties won't be sorted.
- `concentric`: Order properties applying outside the box model, moving inward to intrinsic changes. Unknown properties won't be sorted.

For more detail, please read [https://github.com/Siilwyn/css-declaration-sorter](https://github.com/Siilwyn/css-declaration-sorter).

Note that sorting declarations never guarantees how empty lines are preserved, because once declarations are moved, trivias information are lost.

Default value is `null`.

## Notes

- For all strategies, custom properties (whose name starts with `--`) won't be sorted.
- It will only sort adjacent CSS declarations. For example:

  ```css
  div {
    width: 0;
    height: 0;
    span {}
    min-width: 0;
    min-height: 0;
  }
  ```

  Those declarations above the `span {}` and those declarations below the `span {}` will be sorted separately.

## Example for `"alphabetical"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0vJLFOo5lJQKM9MKcmwUjCwBrIzUjPTM0qgnJTM4oKcxEorhbSc1AprrloAWvsmnjEAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSklNzkksSizJzM%2FzL0pJLVKyUlBKzCnISExKLclMTsxR4qoFAOBP0aAoAAAA&syntax=scss)

```css
div {
  display: flex;
  height: 0;
  width: 0;
}
```

## Example for `"smacss"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0vJLFOo5lJQKM9MKcmwUjCwBrIzUjPTM0qgnJTM4oKcxEorhbSc1AprrloAWvsmnjEAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSklNzkksSizJzM%2FzL0pJLVKyUlAqzk1MLi5W4qoFAGowk7AiAAAA&syntax=scss)

```css
div {
  display: flex;
  width: 0;
  height: 0;
}
```

## Example for `"concentric"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0vJLFOo5lJQKM9MKcmwUjCwBrIzUjPTM0qgnJTM4oKcxEorhbSc1AprrloAWvsmnjEAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSklNzkksSizJzM%2FzL0pJLVKyUlBKzs9LTs0rKcpMVuKqBQA32QzHJgAAAA%3D%3D&syntax=scss)

```css
div {
  display: flex;
  width: 0;
  height: 0;
}
```
