# `selectorOverrideCommentDirective`

Text directive for overriding selector formatting.

This can be used to ignore formatting selector of a specific qualified rule without ignoring the whole qualified rule.
Or, it can be used to format a specific selector in a different way.
For example, you're resetting CSS with many tag name selectors,
then you can use "wrap" style to format this selector only without affecting other selectors.

Suppose [`"blockSelectorLineBreak"`](./block-selector-linebreak.md) is [`"always"`](./block-selector-linebreak.md#example-for-always) in configuration, with the following CSS:

```css
/* malva-selector-override wrap */
html, body, div, span, object, iframe, h1, h2, h3, h4, h5, h6, p, blockquote,
pre, abbr {}

.btn,
.btn-primary {}
```

Note that nested qualified rules won't be affected:

```css
/* malva-selector-override wrap */
html, body, div, span, object, iframe, h1, h2, h3, h4, h5, h6, p, blockquote,
pre, abbr {
  /* nested qualified rule is still respecting configuration, not overriden value */
  .btn,
  .btn-primary {}
}
```

Default is `"malva-selector-override"`.
