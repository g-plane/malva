# `trailingComma`

Control whether trailing comma should be inserted or not.

This only affects Sass list, Sass map, Sass parameters/arguments list,
Less list and Less parameters/arguments list.
CSS functions won't respect this option.

Default value is `false`.

## Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA1NJzs9Ly0y3UtDgUlAoyUjNTS2GsBUUcjOLS2BsBYWM1MSU1CIrBeWU5LTEZAMdqDBQe0lqHlCdsoGBpZlFEkw8LT%2B%2FBKzcwjTZ3DIZIqwJoVSKC4oy89KxmJ1mAjTbHNPsZCMTU5NUDLPT0pIMTS2QzAYSmtYAa6vVzdMAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKilKzMzJzEt3zs%2FNTVSyUkhLzClO5aoFAKpkbbAcAAAA&syntax=scss)

```scss
$config: (
  themes: (
    mist: (header: #dcfac0, content: #00968b, footer: #85c79c),
    $spring: (header: #f4fac7, content: #c2454e, footer: #ffb158)
  )
);
```

## Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA22MMQ6EMAwEe15hCQqQrginBAK8BowNKUhOJP8XgUDFFZZWs%2Bsp0Fk2Sw9lBhBW2sinDLAZH54MsNI4095DPiOPKD43ju%2BBbNzlQnSNnh7OzoVrrhW2HV60SmXhf7uxyx81y6hu32r8SiXppWaeaqWTOjuvGg5FOmEBzwAAAA%3D%3D&config=H4sIAAAAAAAAA6vmUlBQKilKzMzJzEt3zs%2FNTVSyUigpKk3lqgUAw%2BJ2whsAAAA%3D&syntax=scss)

```scss
$config: (
  themes: (
    mist: (header: #dcfac0, content: #00968b, footer: #85c79c),
    $spring: (header: #f4fac7, content: #c2454e, footer: #ffb158),
  ),
);
```
