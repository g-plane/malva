# 🌷 Malva

Malva is a configurable, smart and fast CSS/SCSS/Sass/Less formatter.

## Why?

### Configurable

Malva is configurable. It provides several configuration options so you can
control the code style as you want.

Given the example below:

```css
button.disabled, button:disabled {}
```

This selector is short enough to be put on single line,
so Malva will put it on a single line by default,
instead of forcing it to be splitted into multiple lines.

However, if you prefer putting into multiple lines, you can configure it.

### Smart

Given the example below:

```css
button.disabled/*please use pseudo class as possible*/,button:disabled {}
```

There're comments inside selector. Some formatters will failed to format it, but Malva will format as:

```css
button.disabled /*please use pseudo class as possible*/, button:disabled {}
```

[Try this on playground.](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0sqLSnJz9NLySxOTMpJTdHXKshJTSxOVSgF4oLi1NKUfIXknMTiYoXEYoWC%2FOLiTKAqLX2dJLA2K5g2hepaAIB7SA1JAAAA&config=H4sIAAAAAAAAA6uuBQBDv6ajAgAAAA%3D%3D&syntax=css)

Also, thanks to the [Raffia](https://github.com/g-plane/raffia) parser,
Malva supports cutting edge CSS syntaxes like `@container`.

### Support (indented) Sass

Malva supports indentation-based Sass, not just SCSS.

## Quick Start

### Try It Online

If you just want a quick try, you can try with the [online playground](https://malva-play.vercel.app/).

### dprint

We've provided [dprint](https://dprint.dev/) integration.

Run the command below to add plugin:

```bash
dprint config add g-plane/malva
```

After adding the dprint plugin, update your `dprint.json` and add configuration:

```jsonc
{
    // ...
    "malva": {
        // Malva config comes here
    }
}
```

You can also read [dprint CLI documentation](https://dprint.dev/cli/) for using dprint to format files.

### Standalone CLI

Currently we don't provide standalone CLI.
Leave a comment in [Discussion](https://github.com/g-plane/malva/discussions/1) and let us know if you need it.

## Configuration

Please refer to [Configuration](./docs/config.md).

## FAQ

### I don't like some of code styles. Can I propose to change it?

Malva is not opinionated and is configurable,
and we accept different code styles then switch them with configuration.
But before proposing a code style change, it's better to open a new issue or discussion.

## Credit

Tests come from [Prettier](https://github.com/prettier/prettier/tree/main/tests/format).

## License

MIT License

Copyright (c) 2023-present Pig Fang
