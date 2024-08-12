Malva is a configurable, smart and fast CSS, SCSS, Sass and Less formatter.

## Basic Usage

You can format source code string by using [`format_text`] function.

```rust
use malva::{config::FormatOptions, format_text, Syntax};

let options = FormatOptions::default();
assert_eq!("a {
  color: red;
}
", &format_text("a{color:red}", Syntax::Css, &options).unwrap());
```

For detailed documentation of configuration,
please refer to [Configuration](https://github.com/g-plane/malva/blob/main/docs/config.md) on GitHub.

If there're syntax errors in source code, it will return `Err`:

```rust
use malva::{config::FormatOptions, format_text, Syntax};

let options = FormatOptions::default();
assert!(format_text("a{", Syntax::Css, &options).is_err());
```

## Print AST

If you have already parsed the AST with Raffia,
you can use [`print_stylesheet`] to print it.

Please note that though you have AST,
you still need to provide comments and specify syntax,
also create [`LineBounds`] manually.

```rust
use malva::{config::FormatOptions, print_stylesheet, LineBounds, Syntax};
use raffia::{ast::Stylesheet, ParserBuilder};

let input = "a{color:red}";
let mut comments = vec![];
let mut parser = ParserBuilder::new(input)
    .syntax(Syntax::Css)
    .comments(&mut comments)
    .build();
let stylesheet = parser.parse::<Stylesheet>().unwrap();

let options = FormatOptions::default();
let line_bounds = LineBounds::new(input);
assert_eq!("a {
  color: red;
}
", &print_stylesheet(&stylesheet, &comments, Some(input), line_bounds, Syntax::Css, &options));
```
