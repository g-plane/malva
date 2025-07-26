#![doc = include_str!("../README.md")]

pub mod config;
mod ctx;
mod doc_gen;
mod error;
mod helpers;
mod line_bounds;
mod state;

use crate::{config::FormatOptions, ctx::Ctx, doc_gen::DocGen, state::State};
pub use crate::{error::Error, line_bounds::LineBounds};
pub use raffia::Syntax;
use raffia::{ast::Stylesheet, token::Comment, ParserBuilder, ParserOptions};
use std::path::Path;

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
        Err(error) => {
            let (line, col) = line_bounds.get_line_col(error.span.start);
            return Err(Error::Parser(error, line, col));
        }
    };

    if comments.first().is_some_and(|comment| {
        comment.span.start == 0
            && comment
                .content
                .trim_start()
                .strip_prefix(&options.language.ignore_file_comment_directive)
                .is_some_and(|rest| {
                    rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_whitespace())
                })
    }) {
        Ok(input.to_owned())
    } else {
        Ok(print_stylesheet(
            &stylesheet,
            &comments,
            Some(input),
            line_bounds,
            syntax,
            options,
        ))
    }
}

/// Print the given stylesheet AST.
/// You may use this when you already have the parsed AST.
pub fn print_stylesheet<'a, 's>(
    stylesheet: &'a Stylesheet<'s>,
    comments: &'a [Comment<'s>],
    source: Option<&'s str>,
    line_bounds: LineBounds,
    syntax: Syntax,
    options: &'a FormatOptions,
) -> String {
    use tiny_pretty::{IndentKind, PrintOptions};

    let ctx = Ctx {
        source,
        syntax,
        options: &options.language,
        comments,
        indent_width: options.layout.indent_width,
        line_bounds,
    };
    let state = State {
        keep_decl_name_case: false,
        selector_override: crate::state::SelectorOverride::Unset,
    };
    let doc = stylesheet.doc(&ctx, &state);
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

/// Detect syntax from file extension.
pub fn detect_syntax(path: impl AsRef<Path>) -> Option<Syntax> {
    match path.as_ref().extension().and_then(std::ffi::OsStr::to_str) {
        Some(ext) if ext.eq_ignore_ascii_case("css") => Some(Syntax::Css),
        Some(ext) if ext.eq_ignore_ascii_case("scss") => Some(Syntax::Scss),
        Some(ext) if ext.eq_ignore_ascii_case("sass") => Some(Syntax::Sass),
        Some(ext) if ext.eq_ignore_ascii_case("less") => Some(Syntax::Less),
        _ => None,
    }
}
