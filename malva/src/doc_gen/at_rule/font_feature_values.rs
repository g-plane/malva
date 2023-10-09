use super::super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for FontFamilyName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            FontFamilyName::Str(str) => str.doc(ctx),
            FontFamilyName::Unquoted(unquoted) => unquoted.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for UnquotedFontFamilyName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(self.idents.iter().map(|ident| ident.doc(ctx)), Doc::space())
                .collect(),
        )
    }
}
