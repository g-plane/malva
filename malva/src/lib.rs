//! Malva is a configurable, smart and fast CSS/SCSS/Sass/Less formatter.
//!
//! ## Basic Usage
//!
//! You can format source code string by using [`format_text`] function.
//!
//! ```
//! use malva::{config::FormatOptions, format_text, Syntax};
//!
//! let options = FormatOptions::default();
//! assert_eq!("a {
//!   color: red;
//! }
//! ", &format_text("a{color:red}", Syntax::Css, &options).unwrap());
//! ```
//!
//! If there're syntax errors in source code, it will return `Err`:
//!
//! ```
//! use malva::{config::FormatOptions, format_text, Syntax};
//!
//! let options = FormatOptions::default();
//! assert!(format_text("a{", Syntax::Css, &options).is_err());
//! ```
//!
//! ## Print AST
//!
//! If you have already parsed the AST with Raffia,
//! you can use [`print_stylesheet`] to print it.
//!
//! Please note that though you have AST,
//! you still need to provide comments and specify syntax,
//! also create [`LineBounds`] manually.
//!
//! ```
//! use malva::{config::FormatOptions, print_stylesheet, LineBounds, Syntax};
//! use raffia::{ast::Stylesheet, ParserBuilder};
//!
//! let input = "a{color:red}";
//! let mut comments = vec![];
//! let mut parser = ParserBuilder::new(input)
//!     .syntax(Syntax::Css)
//!     .comments(&mut comments)
//!     .build();
//! let stylesheet = parser.parse::<Stylesheet>().unwrap();
//!
//! let options = FormatOptions::default();
//! let line_bounds = LineBounds::new(input);
//! assert_eq!("a {
//!   color: red;
//! }
//! ", &print_stylesheet(&stylesheet, &comments, line_bounds, Syntax::Css, &options));
//! ```

pub mod config;
mod ctx;
mod doc_gen;
mod error;
mod line_bounds;
mod state;

use crate::{config::FormatOptions, ctx::Ctx, doc_gen::DocGen};
pub use crate::{error::Error, line_bounds::LineBounds};
pub use raffia::Syntax;
use raffia::{ast::Stylesheet, token::Comment, ParserBuilder, ParserOptions};

/// Format the given source code.
pub fn format_text(input: &str, syntax: Syntax, options: &FormatOptions) -> Result<String, Error> {
    let line_bounds = LineBounds::new(input);
    let mut comments = vec![];
    let mut parser = ParserBuilder::new(input)
        .syntax(syntax)
        .comments(&mut comments)
        .options(ParserOptions {
            try_parsing_value_in_custom_property: true,
            tolerate_semicolon_in_sass: true,
        })
        .build();
    let stylesheet = match parser.parse::<Stylesheet>() {
        Ok(stylesheet) => stylesheet,
        Err(error) => return Err(error.into()),
    };

    Ok(print_stylesheet(
        &stylesheet,
        &comments,
        line_bounds,
        syntax,
        options,
    ))
}

/// Print the given stylesheet AST.
/// You may use this when you already have the parsed AST.
pub fn print_stylesheet<'a, 's>(
    stylesheet: &'a Stylesheet<'s>,
    comments: &'a [Comment<'s>],
    line_bounds: LineBounds,
    syntax: Syntax,
    options: &'a FormatOptions,
) -> String {
    use tiny_pretty::{IndentKind, PrintOptions};

    let ctx = Ctx {
        syntax,
        options: &options.language,
        comments,
        indent_width: options.layout.indent_width,
        line_bounds,
        state: Default::default(),
    };
    let doc = stylesheet.doc(&ctx);
    tiny_pretty::print(
        &doc,
        &PrintOptions {
            indent_kind: if options.layout.use_tabs {
                IndentKind::Tab
            } else {
                IndentKind::Space
            },
            line_break: options.layout.line_break.clone().into(),
            width: options.layout.print_width,
            tab_size: options.layout.indent_width,
        },
    )
}
