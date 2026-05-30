# `fontFamilyNames`

Control the layout of font names in `font-family` declaration.

Possible values:

- `"consistent"`: If all font names can be put on a single line, there won't be line breaks; otherwise, there will be line breaks after each comma.
- `"wrap"`: Font names will be put on one line as possible. Once it exceeds [`printWidth`](./print-width.md), line break will be inserted where the code exceeds [`printWidth`](./print-width.md).

Default value is `"consistent"`.

## Example for `"consistent"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAAzXKOwqAMBBF0d5VPKx1A1rZaZNCVzDKBAaSDCRREHHvip%2FucrieJOAoAKsh15a8uL3BEDLH6lZg1Fmzvl327DbOshAMr1x%2B2kUhB6Mb%2FWLEz2vCRCF99DxvplvrxFFsW5wXJiQNK4AAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSsvPK3FLzM3MqfRLzE0tVrJSUErOzyvOLC5JzStR4qoFAIoPP9glAAAA&syntax=css)

```css
main {
  font-family:
    Inter,
    Roboto,
    "Helvetica Neue",
    "Arial Nova",
    "Nimbus Sans",
    Arial,
    sans-serif;
}
```

## Example for `"wrap"`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAAzXKOwqAMBBF0d5VPKx1A1rZaZNCVzDKBAaSDCRREHHvip%2FucrieJOAoAKsh15a8uL3BEDLH6lZg1Fmzvl327DbOshAMr1x%2B2kUhB6Mb%2FWLEz2vCRCF99DxvplvrxFFsW5wXJiQNK4AAAAA%3D&config=H4sIAAAAAAAAA6vmUlBQSsvPK3FLzM3MqfRLzE0tVrJSUCovSixQ4qoFALVe9sQfAAAA&syntax=css)

```css
main {
  font-family:
    Inter, Roboto, "Helvetica Neue", "Arial Nova", "Nimbus Sans", Arial,
    sans-serif;
}
```
