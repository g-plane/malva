# `preferSingleLine`

Control whether items should be placed on single line as possible, even they're originally on multiple lines.

Default value is `false`.

This global option can be overridden by different syntax nodes:

- `selectors.preferSingleLine` ([`blockSelectorLinebreak`](./block-selector-linebreak.md) must be [`"consistent"`](./block-selector-linebreak.md#example-for-consistent), otherwise this will be ignored)
- `functionArgs.preferSingleLine`
- `sassContentAtRule.preferSingleLine`
- `sassIncludeAtRule.preferSingleLine`
- `sassMap.preferSingleLine`
- `sassModuleConfig.preferSingleLine`
- `sassParams.preferSingleLine`
- `lessImportOptions.preferSingleLine`
- `lessMixinArgs.preferSingleLine`
- `lessMixinParams.preferSingleLine`

Given the following example CSS:

```css
a {
  color: rgb(
    0,
    0, 0
  );
}
```

## Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjOz8kvslIoSk%2FSAPIUFAx0oJSCAZChac1VywUAA9K0WikAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKihKTUstCs7MS89J9cnMS1WyUkhLzClO5aoFAEOBhgUfAAAA&syntax=css)

```css
a {
  color: rgb(
    0,
    0,
    0
  );
}
```

## Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA0tUqOZSUEjOz8kvslIoSk%2FSAPIUFAx0oJSCAZChac1VywUAA9K0WikAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQKihKTUstCs7MS89J9cnMS1WyUigpKk3lqgUAfjpc%2BR4AAAA%3D&syntax=css)

```css
a {
  color: rgb(0, 0, 0);
}
```
