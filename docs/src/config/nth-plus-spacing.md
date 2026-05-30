# `nthPlusSpacing`

Control whether there should be spaces around the `+` or `-` operator in "An+B" selector syntax.

Default value is `false`.

## Example for `false`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7PKK8nQTc7IzEnRMMrTNtRUqK7lskIW08Uipm2AKYYpoqCrAFYHAGlHTRJjAAAA&config=H4sIAAAAAAAAA6vmUlBQyivJCMgpLQ4uSEzOzEtXslJIS8wpTuWqBQD5i%2BU7HQAAAA%3D%3D&syntax=css)

```css
:nth-child(2n+1) {}
:nth-child(2n-1) {}
```

## Example for `true`

[Playground](https://malva-play.vercel.app/?code=H4sIAAAAAAAAA7PKK8nQTc7IzEnRMMrTNtRUqK7lskIW08Uipm2AKYYpoqCrAFYHAGlHTRJjAAAA&config=H4sIAAAAAAAAA6vmUlBQyivJCMgpLQ4uSEzOzEtXslIoKSpN5aoFAJlbIfQcAAAA&syntax=css)

```css
:nth-child(2n + 1) {}
:nth-child(2n - 1) {}
```
