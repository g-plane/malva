# `linebreakInPseudoParens`

Control whether line break should be inserted in pseudo class/element parens
or not if current line is too long.

Default value is `false`.

## Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7Mqz0gtStVQLjCwSs5ITc5OTVGoU1AuyEmsROYnG8J4OkBJQ7xKjZCVGuFV6g%2FjaSpU1wIAa839xooAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQysnMS00qSk3M9swLKE4tTckPSCxKzStWslJIS8wpTuWqBQAAWvxOJgAAAA%3D%3D&syntax=css)

```css
:where(#p0:checked ~ #play:checked ~ #c1:checked, #p1:checked
  ~ #play:checked
  ~ #c2:checked, #p2:checked ~ #play:checked ~ #cO:checked) {}
```

## Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7Mqz0gtStVQLjCwSs5ITc5OTVGoU1AuyEmsROYnG8J4OkBJQ7xKjZCVGuFV6g%2FjaSpU1wIAa839xooAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQysnMS00qSk3M9swLKE4tTckPSCxKzStWslIoKSpN5aoFAFs%2BU4clAAAA&syntax=css)

```css
:where(
  #p0:checked ~ #play:checked ~ #c1:checked,
    #p1:checked ~ #play:checked ~ #c2:checked,
    #p2:checked ~ #play:checked ~ #cO:checked
) {}
```
