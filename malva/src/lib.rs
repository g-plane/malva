pub mod config;
mod ctx;
mod doc_gen;
mod error;

use crate::{config::FormatOptions, ctx::Ctx};
use doc_gen::DocGen;
pub use error::Error;
pub use raffia::Syntax;
use raffia::{ast::Stylesheet, token::Comment, ParserBuilder};

pub fn format_text(input: &str, syntax: Syntax, options: &FormatOptions) -> Result<String, Error> {
    let mut comments = vec![];
    let mut parser = ParserBuilder::new(&input).comments(&mut comments).build();
    let stylesheet = parser.parse::<Stylesheet>().map_err(Error::from)?;
    print_stylesheet(&stylesheet, &comments, syntax, options)
}

pub fn print_stylesheet(
    stylesheet: &Stylesheet,
    comments: &[Comment],
    syntax: Syntax,
    options: &FormatOptions,
) -> Result<String, Error> {
    let ctx = Ctx {
        syntax,
        options: &options.language,
        comments: &comments,
        indent_width: options.layout.indent_width,
    };
    let doc = stylesheet.doc(&ctx);
    tiny_pretty::print(&doc, &Default::default()).map_err(Error::from)
}
